import { auth } from "@/lib/auth";
import { NextResponse } from "next/server";

export default auth((req) => {
  const isLoggedIn = !!req.auth;
  const { pathname } = req.nextUrl;

  const isLoginRoute = pathname === "/admin/login";
  const isAdminRoute = pathname.startsWith("/admin") && !isLoginRoute;

  if (isAdminRoute && !isLoggedIn) {
    const loginUrl = new URL("/admin/login", req.nextUrl.origin);
    loginUrl.searchParams.set("callbackUrl", pathname);
    return NextResponse.redirect(loginUrl);
  }

  if (isLoginRoute && isLoggedIn) {
    return NextResponse.redirect(new URL("/admin", req.nextUrl.origin));
  }
});

export const config = {
  matcher: ["/admin/:path*"],
};
