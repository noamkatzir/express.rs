const express = require('express')
// const compression = require('compression')
const app = express()

// app.use(compression({
//     filter: (req, res) => {
//         if (req.headers['x-no-compression']) {
//         // don't compress responses with this request header
//         return false
//         }
    
//         // fallback to standard filter function
//         return compression.filter(req, res)
//     } 
// }))

app.get('/json', (req, res) => res.json({key:"value1", value: {key:"value1", value: {key:"value1", value: {key:"value1"}}}}))
app.get('/file', (req, res) => res.sendFile("/home/noam/dev/workspace/express/src/bin/temp.txt"))

app.listen(3000, () => console.log('Example app listening on port 3000!'))