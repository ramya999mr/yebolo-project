import { useEffect, useState } from "react";
import { LineChart, Line, XAxis, YAxis, Tooltip, Legend } from "recharts";

export default function Home() {
  const [data, setData] = useState<any[]>([]);

  useEffect(() => {
    // Placeholder for WebSocket or API connection
    setData([
      { time: "10:00", price: 50000, rsi: 55 },
      { time: "10:05", price: 50050, rsi: 60 },
      { time: "10:10", price: 49980, rsi: 45 }
    ]);
  }, []);

  return (
    <div style={{ padding: 20 }}>
      <h1>Crypto RSI Dashboard</h1>
      <LineChart width={800} height={400} data={data}>
        <XAxis dataKey="time" />
        <YAxis />
        <Tooltip />
        <Legend />
        <Line type="monotone" dataKey="price" stroke="#8884d8" />
        <Line type="monotone" dataKey="rsi" stroke="#82ca9d" />
      </LineChart>
    </div>
  );
}