// script.js - IoT Dashboard Behavior

const TEMP_ENDPOINT = "/api/temperature";
const HUM_ENDPOINT = "/api/humidity";
const LED_ENDPOINT = "/api/led-toggle";

let lastTemp = null;
let lastHum = null;
let ledOn = false;

// Fetch sensor data and update UI
async function fetchSensorData() {
  try {
    const [tempRes, humRes] = await Promise.all([
      fetch(TEMP_ENDPOINT),
      fetch(HUM_ENDPOINT)
    ]);

    if (!tempRes.ok || !humRes.ok) {
      console.error("Failed to fetch sensor data");
      return;
    }

    const tempJson = await tempRes.json();
    const humJson = await humRes.json();

    updateCard("temp", tempJson.value, lastTemp);
    updateCard("hum", humJson.value, lastHum);

    lastTemp = tempJson.value;
    lastHum = humJson.value;
  } catch (err) {
    console.error("Error fetching sensor data:", err);
  }
}

// Update a sensor card with value and trend arrow
function updateCard(type, value, lastValue) {
  const valueEl = document.getElementById(`${type}-value`);
  const trendEl = document.getElementById(`${type}-trend`);

  valueEl.textContent = `${value.toFixed(1)} ${type === 'temp' ? '°C' : '%'}`;

  if (lastValue !== null) {
    if (value > lastValue) {
      trendEl.textContent = '▲ Rising';
      trendEl.className = 'trend up';
    } else if (value < lastValue) {
      trendEl.textContent = '▼ Falling';
      trendEl.className = 'trend down';
    } else {
      trendEl.textContent = '▶ Stable';
      trendEl.className = 'trend stable';
    }
  }
}

// Toggle LED state via API
async function toggleLed() {
  try {
    const res = await fetch(LED_ENDPOINT, { method: 'POST' });
    if (!res.ok) throw new Error('Network response was not ok');

    const json = await res.json();
    ledOn = json.ledOn;
    document.getElementById('led-status').textContent = `LED is ${ledOn ? 'ON' : 'OFF'}`;
  } catch (err) {
    console.error('Error toggling LED:', err);
  }
}

// Attach event listener to button
document.getElementById('toggle-led').addEventListener('click', () => {
  toggleLed();
});

// Initial fetch and set interval
fetchSensorData();
setInterval(fetchSensorData, 5000);