const express = require("express");
const cors = require("cors");
const bodyParser = require("body-parser");
const fs = require("fs");
const path = require("path");

const app = express();
const port = 3001;

app.use(cors());
app.use(bodyParser.json());

const jsonFilePath = path.join(__dirname, "huskeliste.json");

app.get("/todos", (req, res) => {
  fs.readFile(jsonFilePath, "utf-8", (err, data) => {
    if (err) {
      console.error(err);
      res.status(500).send({ error: "An error occurred reading the file." });
    } else {
      res.send(JSON.parse(data));
    }
  });
});

app.put("/todos", (req, res) => {
  fs.writeFile(jsonFilePath, JSON.stringify(req.body), (err) => {
    if (err) {
      console.error(err);
      res.status(500).send({ error: "An error occurred writing to the file." });
    } else {
      res.send({ message: "File updated successfully." });
    }
  });
});

app.listen(port, () => {
  console.log(`Server listening at http://localhost:${port}`);
});
