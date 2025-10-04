const { Kafka } = require("kafkajs");
const fs = require("fs");
const csv = require("csv-parser");

const kafka = new Kafka({
  clientId: "trade-ingestion",
  brokers: ["localhost:9092"],
});

const producer = kafka.producer();

async function run() {
  await producer.connect();
  fs.createReadStream("../trades_data.csv")
    .pipe(csv())
    .on("data", async (row) => {
      await producer.send({
        topic: "trade-data",
        messages: [{ value: JSON.stringify(row) }],
      });
      console.log("Published:", row);
    })
    .on("end", () => {
      console.log("CSV ingestion finished");
    });
}

run().catch(console.error);