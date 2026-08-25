const express = require('express');
const cors = require('cors');

const app = express();
const PORT = 3000;

// Beveiliging: sta alleen requests toe vanaf jouw specifieke frontend domein
// Voor lokaal testen staat dit op poort 8080, pas dit aan naar je uiteindelijke domein
app.use(cors({
    origin: 'http://localhost:8080' 
}));

app.get('/api/feed', async (req, res) => {
    const rssUrl = 'https://www.omnycontent.com/d/playlist/61ee9ca4-a1b2-4660-9651-b2b70035edf5/d643a93a-f161-486c-b8cc-b31501095860/969874ed-2240-43b1-bf81-b3150109586e/podcast.rss';
    
    try {
        // Maakt gebruik van de native fetch API (beschikbaar in Node.js 18+)
        const response = await fetch(rssUrl);
        if (!response.ok) throw new Error('Kon de RSS feed niet bereiken.');
        
        const xmlData = await response.text();
        
        // Stuur de data door als correcte XML
        res.header('Content-Type', 'application/xml');
        res.send(xmlData);
    } catch (error) {
        console.error('Fout bij ophalen feed:', error);
        res.status(500).json({ error: 'Interne serverfout bij het ophalen van de feed.' });
    }
});

app.listen(PORT, () => {
    console.log(`Beveiligde RSS backend draait op http://localhost:${PORT}`);
});