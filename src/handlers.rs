use std::time::SystemTime;

use crate::models::BankTransaction;
use crate::{DBConnection, Result};

use chrono::{DateTime, Utc};

use serde_json::json;
use warp::{http::StatusCode, reply, Reply};

pub async fn add_client_transaction(
    client_id: i32,
    new_transaction: BankTransaction,
    db: DBConnection,
) -> Result<impl Reply> {
    // hard-coded clients id
    if (1..6).contains(&client_id) {
        // check for transaction fields
        if !new_transaction.is_valid() {
            return Ok(reply::with_status(
                reply::json(&format!("Invalid Transaction {:?}", new_transaction)),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
        let select_for_update = r#"
            SELECT
                limit_amount,
                balance
            FROM accounts
            WHERE accounts.id = $1
            FOR UPDATE
        "#;
        // println!("[INFO] DBPOOL {:?}", db.lock().await.status());
        let mut db_client = db.lock().await.get().await.unwrap();
        let db_transaction = db_client.transaction().await.unwrap();
        let account = db_transaction.query_one(select_for_update, &[&client_id]).await.unwrap();
        let limit_amount: i32 = account.get("limit_amount");
        let mut balance: i32 = account.get("balance");
        // check for transaction feasibility
        if new_transaction.tipo == "d"
            && insuficient_limit(balance, new_transaction.valor, limit_amount)
        {
            db_transaction.commit().await.unwrap();
            return Ok(reply::with_status(
                reply::json(&format!(
                    "Limit reached. Balance = {balance} Transaction amount = {} Limit = {limit_amount}",
                    new_transaction.valor
                )),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
        let insert = r#"
            INSERT INTO transactions (account_id, amount, transaction_type, description)
            VALUES ($1, $2, $3, $4)
            "#;
        let _ = db_transaction
            .execute(
                insert,
                &[
                    &client_id,
                    &new_transaction.valor,
                    &new_transaction.tipo,
                    &new_transaction.descricao,
                ],
            )
            .await
            .unwrap();
        let update_account = r#"
            UPDATE accounts 
            SET balance = balance + $2
            WHERE accounts.id = $1
        "#;
        if new_transaction.tipo == "d" {
            db_transaction
                .execute(update_account, &[&client_id, &-new_transaction.valor])
                .await
                .unwrap();
            balance -= new_transaction.valor;
        } else {
            db_transaction
                .execute(update_account, &[&client_id, &new_transaction.valor])
                .await
                .unwrap();
            balance += new_transaction.valor;
        }
        //
        // let just_select = r#"
        //     SELECT
        //         limit_amount,
        //         balance
        //     FROM accounts
        //     WHERE accounts.id = $1
        // "#;
        // let account = db_transaction.query_one(just_select, &[&client_id]).await.unwrap();
        // let balance: i32 = account.get("balance");
        //
        db_transaction.commit().await.unwrap();
        let body = json!({
            "limite" : limit_amount,
            "saldo" : balance
        });
        return Ok(reply::with_status(reply::json(&body), StatusCode::OK));
    }
    Ok(reply::with_status(
        reply::json(&format!("Client {} not found", client_id)),
        StatusCode::NOT_FOUND,
    ))
}

pub async fn get_client_balance(client_id: i32, db: DBConnection) -> Result<impl Reply> {
    // hard-coded clients id
    if client_exists(&client_id) {
        let account_query = r#"
            SELECT 
                limit_amount,
                balance
            FROM accounts
            WHERE accounts.id = $1
        "#;
        let mut db_client = db.lock().await.get().await.unwrap();
        let db_transaction = db_client.transaction().await.unwrap();
        let account = db_transaction
            .query_one(account_query, &[&client_id])
            .await
            .unwrap();
        let limit_amount: i32 = account.get("limit_amount");
        let balance: i32 = account.get("balance");
        let transaction_query = r#"
            SELECT 
                amount,
                transaction_type,
                description,
                date
            FROM transactions
            WHERE account_id = $1
            ORDER BY date DESC
            LIMIT 10
        "#;
        let last_transactions: Vec<_> = db_transaction
            .query(transaction_query, &[&client_id])
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                let amount: i32 = row.get("amount");
                let transaction_type: &str = row.get("transaction_type");
                let description: &str = row.get("description");
                let date: DateTime<Utc> = row.get::<&str, SystemTime>("date").into();
                json!({"valor": amount,
                    "tipo": transaction_type,
                    "descricao": description,
                    "realizada_em": date
                })
            })
            .collect();
        let body = json!({
            "saldo": json!({
                "limite": limit_amount,
                "total": balance,
                "data_extrato": Utc::now().to_string()
            }),
            "ultimas_transacoes": last_transactions
        });
        db_transaction.commit().await.unwrap();
        return Ok(reply::with_status(reply::json(&body), StatusCode::OK));
    }
    Ok(reply::with_status(
        reply::json(&"Client Not Exists"),
        StatusCode::NOT_FOUND,
    ))
}

fn insuficient_limit(actual_balance: i32, transaction_amount: i32, limit: i32) -> bool {
    (actual_balance - transaction_amount) < -limit
}

fn client_exists(id_cliente: &i32) -> bool {
    vec![1, 2, 3, 4, 5].contains(id_cliente)
}
