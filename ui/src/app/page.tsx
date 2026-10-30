export default function Home() {
  return (
    <main className="flex min-h-screen items-center justify-center bg-slate-950 p-6 text-white">
      <section className="w-full max-w-lg rounded-3xl border border-slate-700 bg-slate-900 p-10 text-center shadow-2xl">
        <div className="mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-2xl bg-blue-600 text-4xl font-bold">
          M
        </div>

        <p className="mb-3 text-sm font-medium uppercase tracking-widest text-blue-400">
          My First Desktop App
        </p>

        <h1 className="text-4xl font-bold tracking-tight">
          Hello From Menghor
        </h1>

        <p className="mt-4 leading-7 text-slate-400">
          Welcome to my desktop application,
          built with Next.js, Tailwind CSS, and Tauri.
        </p>

        <div className="mt-8 inline-flex items-center gap-2 rounded-full bg-emerald-400/10 px-4 py-2 text-sm text-emerald-400">
          <span className="h-2 w-2 rounded-full bg-emerald-400" />
          Ready to use
        </div>
      </section>
    </main>
  );
}
