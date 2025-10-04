use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::Message;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), KafkaError> {
    // Get broker address from ENV or fallback
    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or("redpanda:9092".to_string());

    println!("🚀 Starting consumer, connecting to broker: {}", brokers);

    // Create consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("group.id", "trade-consumer")
        .set("auto.offset.reset", "earliest") // read from beginning
        .create()
        .expect("❌ Failed to create consumer");

    // Subscribe to topic
    consumer
        .subscribe(&["trades"])
        .expect("❌ Failed to subscribe to topic 'trades'");

    println!("📡 Listening for messages on topic 'trades'...");

    let mut stream = consumer.stream();

    // Process messages
    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => {
                let payload = message
                    .payload_view::<str>()
                    .unwrap_or(Ok("<Invalid UTF-8>"));
                println!("📥 Received: {:?}", payload);

                if let Some(headers) = message.headers() {
                    println!(" ↳ Headers: {:?}", headers);
                }
            }
            Err(e) => eprintln!("⚠️ Kafka error: {}", e),
        }
    }

    Ok(())
}