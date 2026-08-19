export default function Footer() {
  return (
    <footer className="relative overflow-hidden bg-black px-5 sm:px-8 lg:px-12 pt-6">
      <div className="relative mx-auto max-w-375">
        <div className="relative overflow-hidden">
          <h2
            aria-label="Atronsa"
            className="select-none whitespace-nowrap text-center font-title text-[25vw] font-medium leading-[0.68] tracking-[-0.085em] text-transparent [-webkit-text-stroke:1px_rgba(255,255,255,0.28)] sm:text-[23vw] sm:[-webkit-text-stroke:1px_rgba(255,255,255,0.3)] lg:text-[21vw]"
          >
            Atronsa
          </h2>
          <div className="pointer-events-none absolute inset-x-0 bottom-0 h-16 bg-linear-to-t from-black to-transparent" />
        </div>

        <div className="relative z-10 mt-2 flex flex-col items-center justify-between gap-4 sm:gap-3 border-t border-white/8 py-5 sm:flex-row">
          <p className="font-poppins text-[11px] sm:text-xs tracking-wide text-white/40">
            Software — Web Design & Development Agency
          </p>
          <p className="font-poppins text-[11px] sm:text-xs tracking-wide text-white/40">
            © {new Date().getFullYear()} Atronsa. All rights reserved.
          </p>
        </div>
      </div>
    </footer>
  );
}
