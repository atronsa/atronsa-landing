import Link from "next/link";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Page Not Found",
  description: "The page you are looking for does not exist.",
  robots: {
    index: false,
    follow: false,
  },
};

export default function NotFound() {
  return (
    <div className="flex flex-col flex-1 items-center justify-center px-8 bg-zinc-50 dark:bg-black font-montserrat">
      <p className="text-xs md:text-sm lg:text-base font-medium text-center leading-relaxed">
        Not Found.{" "}
        <span className="border-b-2">
          <Link href="/">Back To Home.</Link>
        </span>
      </p>
    </div>
  );
}
