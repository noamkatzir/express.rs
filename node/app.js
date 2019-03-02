const express = require('express')
const app = express()

app.get('/', (req, res) => res.json({key:"value1", value: {key:"value1", value: {key:"value1", value: {key:"value1"}}}}))

app.listen(3000, () => console.log('Example app listening on port 3000!'))