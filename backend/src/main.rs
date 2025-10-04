use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Trade {
    token: String,
    price: f64,
    quantity: f64,
    timestamp: String,
}

fn calculate_rsi(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period { return 50.0; }
    let mut gains = 0.0;
    let mut losses = 0.0;

    for i in 1..=period {
        let diff = prices[prices.len()-i] - prices[prices.len()-i-1];
        if diff > 0.0 { gains += diff; } else { losses -= diff; }
    }

    if losses == 0.0 { return 100.0; }
    let rs = gains / losses;
    100.0 - (100.0 / (1.0 + rs))
}

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rsi-group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["trade-data"]).unwrap();

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    let mut prices: Vec<f64> = Vec::new();

    let mut stream = consumer.stream();
    while let Some(message) = stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload_view::<str>().ok().flatten() {
                    if let Ok(trade) = serde_json::from_str::<Trade>(payload) {
                        prices.push(trade.price);
                        let rsi = calculate_rsi(&prices, 14);
                        let rsi_data = serde_json::to_string(&(trade.token.clone(), rsi)).unwrap();
                        let _ = producer.send(
                            FutureRecord::to("rsi-data").payload(&rsi_data),
                            0
                        ).await;
                        println!("{} -> RSI: {}", trade.token, rsi);
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}