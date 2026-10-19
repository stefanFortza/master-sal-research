use std::arch::x86_64::{__rdtscp, _mm_lfence, _rdtsc};

pub unsafe fn measure_cycles<F: FnOnce()>(f: F) -> u64 {
    _mm_lfence(); // Asigură-te că toate instrucțiunile anterioare s-au terminat
    let start = _rdtsc(); // Citește timestamp-ul de început

    f(); // Execută funcția

    _mm_lfence(); // Asigură-te că toate instrucțiunile din f() s-au terminat
    let end = _rdtsc(); // Citește timestamp-ul de sfârșit
    // end - start // Returnează diferența în cicluri
    end.saturating_sub(start)
}
