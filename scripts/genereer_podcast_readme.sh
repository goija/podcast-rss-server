#!/bin/bash
set +H

echo "📝 VORTEX - Genereren en pushen van README.md voor podcast-rss-server..."

# Zoek de podcast-rss-server map
TARGET_DIR="podcast-rss-server"
if [ ! -d "$TARGET_DIR" ]; then
    mkdir -p "$TARGET_DIR"
fi

cd "$TARGET_DIR" || exit

# Maak de schone README.md aan
cat << 'README_EOF' > README.md
# 🎙️ VORTEX Sovereign Podcast RSS Server

Een lichtgewicht, snelle en soevereine RSS-podcastserver geschreven in **Rust** met behulp van het **Axum** framework. Deze server scant automatisch een lokale map op `.mp3`-bestanden, leidt schone titels af van de bestandsnamen en genereert dynamisch een valide RSS 2.0-feed.

---

## 🚀 Kenmerken
- **Bare-Metal & Snel:** Gebouwd in Rust (Axum + Tokio) voor maximale performance en minimale resource-belasting.
- **Automatische RSS Generatie:** Zet direct mp3-bestanden om in een geldige XML-feed (`/rss.xml`).
- **Statische Bestandslevering:** Biedt de audiobestanden direct aan via de ingebouwde static file service (`/podcasts/...`).
- **Soeverein & Self-Hosted:** Geen externe cloud-afhankelijkheden; draait volledig lokaal op eigen hardware (bijv. Tiny Server / Debian / Raspberry Pi).

---

## 🛠️ Installatie & Lokaal Starten

1. **Clone de repository:**
   ```bash
   git clone [https://github.com/goija/podcast-rss-server.git](https://github.com/goija/podcast-rss-server.git)
   cd podcast-rss-server
mkdir -p podcasts
# Plaats hier je .mp3 bestanden
cargo run --release
sudo nano /etc/systemd/system/podcast-rss.service
[Unit]
Description=VORTEX Rust RSS Podcast Server
After=network.target

[Service]
Type=simple
User=ontwerp
WorkingDirectory=/pad/naar/podcast-rss-server
ExecStart=/pad/naar/podcast-rss-server/target/release/podcast-rss-server
Restart=always

[Install]
WantedBy=multi-user.target
sudo systemctl daemon-reload
sudo systemctl enable podcast-rss.service
sudo systemctl start podcast-rss.service
README_EOF

echo "✅ README.md succesvol aangemaakt in $TARGET_DIR."
git add README.md
git commit -m "docs(readme): professionele readme.md toegevoegd voor podcast-rss-server" || true
git pull origin main --rebase || true
git push origin main

echo "🎉 De README.md is succesvol gepusht naar https://github.com/goija/podcast-rss-server!"
