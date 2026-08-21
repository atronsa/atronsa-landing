import Link from "next/link";
import { auth, signOut } from "@/lib/auth";
import { redirect } from "next/navigation";
import { prisma } from "@/lib/prisma";
import MessageTable from "@/components/ui/MessageTable";

export const dynamic = "force-dynamic";

const PAGE_SIZE = 10;

export default async function AdminPage({
  searchParams,
}: {
  searchParams: Promise<{ page?: string; search?: string }>;
}) {
  const session = await auth();
  if (!session) redirect("/admin/login");

  const params = await searchParams;
  const page = Math.max(1, Number(params.page) || 1);
  const search = params.search?.trim() || "";
  const skip = (page - 1) * PAGE_SIZE;

  const where = search
    ? {
        OR: [
          { name: { contains: search } },
          { email: { contains: search } },
          { phoneNumber: { contains: search } },
          { message: { contains: search } },
          { projectType: { contains: search } },
        ],
      }
    : {};

  const [messages, total] = await Promise.all([
    prisma.contactMessage.findMany({
      where,
      orderBy: { createdAt: "desc" },
      skip,
      take: PAGE_SIZE,
    }),
    prisma.contactMessage.count({ where }),
  ]);

  const totalPages = Math.max(1, Math.ceil(total / PAGE_SIZE));

  const getVisiblePages = () => {
    if (totalPages <= 5) {
      return Array.from({ length: totalPages }, (_, i) => i + 1);
    }

    if (page <= 3) {
      return [1, 2, 3, 4, 5];
    }

    if (page >= totalPages - 2) {
      return [
        totalPages - 4,
        totalPages - 3,
        totalPages - 2,
        totalPages - 1,
        totalPages,
      ];
    }

    return [page - 2, page - 1, page, page + 1, page + 2];
  };

  const buildHref = (p: number) => {
    const urlParams = new URLSearchParams();
    urlParams.set("page", String(p));
    if (search) urlParams.set("search", search);
    return `/admin?${urlParams.toString()}`;
  };

  const visiblePages = getVisiblePages();

  return (
    <main className="relative min-h-screen overflow-hidden bg-black px-4 sm:px-6 lg:px-8 pt-24 sm:pt-28 lg:pt-32 pb-10">
      <div
        className="pointer-events-none absolute inset-0 opacity-30"
        style={{
          backgroundImage: `
            linear-gradient(rgba(255,255,255,0.025) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.025) 1px, transparent 1px)
          `,
          backgroundSize: "80px 80px",
          maskImage:
            "linear-gradient(to bottom, transparent, black 15%, black 85%, transparent)",
          WebkitMaskImage:
            "linear-gradient(to bottom, transparent, black 15%, black 85%, transparent)",
        }}
      />

      <div className="relative z-10 mx-auto w-full max-w-6xl">
        <div className="flex flex-col gap-4 border-b border-white/[0.07] pb-6 md:flex-row md:items-center md:justify-between">
          <div className="min-w-0">
            <h1 className="font-title text-2xl font-medium leading-[0.9] tracking-[-0.055em] text-white sm:text-3xl lg:text-4xl">
              Contact Inquiries
            </h1>
            <p className="mt-2 font-poppins text-[10px] text-white/40 sm:text-xs">
              {total} total {total === 1 ? "message" : "messages"}
              {search && (
                <>
                  {" "}
                  for{" "}
                  <span className="text-white/70">&quot;{search}&quot;</span>
                </>
              )}
            </p>
          </div>

          <div className="flex flex-wrap items-center gap-2">
            <Link
              href="/admin"
              className="flex h-9 w-9 cursor-pointer items-center justify-center rounded-xl border border-white/10 bg-white/4 transition-all duration-300 hover:border-white/20 hover:bg-white/8"
              aria-label="Refresh"
              title="Reset search and pagination"
            >
              <svg
                className="h-3.5 w-3.5 text-white/50 transition-transform duration-300 hover:rotate-180 hover:text-white"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
                />
              </svg>
            </Link>

            <form
              className="flex items-center gap-2"
              action="/admin"
              method="GET"
            >
              <div className="relative min-w-0 flex-1 sm:flex-none">
                <svg
                  className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-white/30"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={1.5}
                    d="M21 21l-4.35-4.35M17 11a6 6 0 11-12 0 6 6 0 0112 0z"
                  />
                </svg>
                <input
                  type="text"
                  name="search"
                  defaultValue={search}
                  placeholder="Search..."
                  className="w-full rounded-xl border border-white/10 bg-white/[0.035] py-2 pl-9 pr-3 font-poppins text-[11px] text-white outline-none placeholder:text-white/25 transition-all duration-300 focus:border-white/30 sm:w-44"
                />
              </div>

              <button
                type="submit"
                className="shrink-0 cursor-pointer rounded-xl border border-white/10 bg-white/4 px-3.5 py-2 font-poppins text-[11px] text-white/60 transition-all duration-300 hover:border-white/20 hover:bg-white/8 hover:text-white"
              >
                Search
              </button>
            </form>

            {/* Sign out */}
            <form
              action={async () => {
                "use server";
                await signOut({ redirectTo: "/admin/login" });
              }}
              className="shrink-0"
            >
              <button
                type="submit"
                className="cursor-pointer rounded-xl border border-white/10 bg-white/4 px-3.5 py-2 font-poppins text-[10px] uppercase tracking-widest text-white/50 transition-all duration-300 hover:border-white/25 hover:bg-white/10 hover:text-white"
              >
                Sign out
              </button>
            </form>
          </div>
        </div>

        {/* Table */}
        <div className="mt-6 overflow-hidden rounded-xl border border-white/8 bg-white/2.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.07)] backdrop-blur-2xl">
          <table className="w-full text-left">
            <thead className="hidden md:table-header-group">
              <tr className="border-b border-white/8 bg-white/2 text-[10px] uppercase tracking-[0.12em] text-white/35">
                <th className="px-4 py-4 font-poppins font-medium lg:px-5">
                  Name
                </th>
                <th className="px-4 py-4 font-poppins font-medium lg:px-5">
                  Email
                </th>
                <th className="px-4 py-4 font-poppins font-medium lg:px-5">
                  Phone
                </th>
                <th className="px-4 py-4 font-poppins font-medium lg:px-5">
                  Service
                </th>
                <th className="px-4 py-4 font-poppins font-medium lg:px-5">
                  Message
                </th>
                <th className="px-4 py-4 font-poppins font-medium lg:px-5">
                  Date
                </th>
              </tr>
            </thead>
            <MessageTable messages={messages} />
          </table>
        </div>

        {/* Pagination */}
        {totalPages > 1 && (
          <div className="mt-6 flex flex-wrap items-center justify-center gap-1.5 sm:gap-2">
            <Link
              href={buildHref(Math.max(1, page - 1))}
              className={`rounded-xl border px-3 py-2 font-poppins text-[11px] transition-all duration-300 sm:px-3.5 ${
                page === 1
                  ? "pointer-events-none border-white/5 bg-white/2 text-white/25"
                  : "border-white/10 bg-white/[0.035] text-white/60 hover:border-white/20 hover:bg-white/8 hover:text-white"
              }`}
            >
              Prev
            </Link>

            {visiblePages.map((p) => (
              <Link
                key={p}
                href={buildHref(p)}
                className={`rounded-xl border px-3 py-2 font-poppins text-[11px] transition-all duration-300 sm:px-3.5 ${
                  p === page
                    ? "border-white/20 bg-white/10 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.1)]"
                    : "border-white/10 bg-white/[0.035] text-white/60 hover:border-white/20 hover:bg-white/8 hover:text-white"
                }`}
              >
                {p}
              </Link>
            ))}

            <Link
              href={buildHref(Math.min(totalPages, page + 1))}
              className={`rounded-xl border px-3 py-2 font-poppins text-[11px] transition-all duration-300 sm:px-3.5 ${
                page === totalPages
                  ? "pointer-events-none border-white/5 bg-white/2 text-white/25"
                  : "border-white/10 bg-white/[0.035] text-white/60 hover:border-white/20 hover:bg-white/8 hover:text-white"
              }`}
            >
              Next
            </Link>
          </div>
        )}
      </div>
    </main>
  );
}
