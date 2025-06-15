import Link from "next/link";

export default function NotFound() {
    return (
        <div className="flex flex-col items-center justify-center h-screen">
            <h1 className="text-4xl font-bold">Constellation</h1>
            <p className="text-lg">Page not found</p>
            <Link
              href="/"
              className="relative border border-gray-500 rounded-md p-2 mt-4 overflow-hidden group"
            >
              <span className="relative z-10">Go back to home</span>
              <span className="absolute inset-0 bg-khaki transform -translate-x-full group-hover:translate-x-0 transition-transform duration-300 ease-in-out"></span>
            </Link>
        </div>
  );
}
