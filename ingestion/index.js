const { Kafka } = require("kafkajs");
const fs = require("fs");
const csv = require("csv-parser");

const kafka = new Kafka({
  clientId: "trade-ingestion",
  brokers: [process.env.KAFKA_BROKERS||"redpanda:9092"],
});

const producer = kafka.producer();

async function run() {
  await producer.connect();
  const filePath=process.argv[2]||"trades_data.csv";
  fs.createReadStream(filePath)
    .pipe(csv())
    .on("data", async (row) => {
      await producer.send({
        topic: "trades",
        messages: [{ value: JSON.stringify(row) }],
      });
      console.log("Published:", row);
    })
    .on("end", () => {
      console.log("CSV ingestion finished");
    });
}

run().catch(console.error);