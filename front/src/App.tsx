import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from './assets/Vite.svg'
import './App.css'

const uri = 'ws://' + 'localhost:3030' + '/ws'
const ws = new WebSocket(uri)

export const App = () => {
	const [count, setCount] = useState(0)

	useEffect(() => {
		ws.onopen = () => console.log('connected')
		ws.onmessage = (msg) => setCount(Number(msg.data))
		ws.onclose = () => console.log('disconnected')
	}, [])

	const onCountClick = () => setCount((count) => count + 1)

	useEffect(() => {
		if (count > 0 && ws.readyState === ws.OPEN) ws.send(String(count))
	}, [count])

	return (
		<div className="app">
			<div>
				<a href="https://vitejs.dev" target="_blank">
					<img src={viteLogo} className="logo" alt="Vite logo" />
				</a>
				<a href="https://reactjs.org" target="_blank">
					<img src={reactLogo} className="logo react" alt="React logo" />
				</a>
			</div>
			<h1>Vite + React</h1>
			<div className="card">
				<button onClick={onCountClick}>count is {count}</button>
			</div>
		</div>
	)
}
