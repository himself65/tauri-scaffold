const PORT = +process.env.PORT || 3000
const app = require('express')()
const jsonParser = require('body-parser').json()

app.post('/hello', jsonParser, (req, res) => {
  res.write(`hello ${req.body.id}`)
  res.end()
})

app.listen(PORT, () => {
  console.log(`http://localhost:${PORT}`)
})
