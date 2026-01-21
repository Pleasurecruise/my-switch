import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { Button } from "@my-monorepo/ui/components/button";
import { trpc } from "./lib/trpc";

function App() {
	const [count, setCount] = useState(0);
	const { data: helloData, isLoading } = useQuery(
		trpc.hello.greet.queryOptions(),
	);

	return (
		<main className="flex min-h-screen flex-col items-center justify-center gap-4">
			<div className="p-4 border rounded">
				<h2 className="text-lg font-bold mb-2">tRPC Test</h2>
				{isLoading ? <p>Loading...</p> : <p>{helloData?.message}</p>}
			</div>

			<Button type="button" onClick={() => setCount((c) => c + 1)}>
				Add 1 to {count}?
			</Button>
		</main>
	);
}

export default App;
