const PORT = +process.env.MOCK_PORT || 3000
const app = require('express')()

app.post('/hello', (req, res) => {
  const { id } = req.body
  res.write(`hello ${id}`)
  res.end()
})

app.listen(PORT, () => {
  console.log(`http://localhost:${PORT}`)
})
