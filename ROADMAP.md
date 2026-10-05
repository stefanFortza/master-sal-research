# Master SAL — Plan Strategic de Lucru & Cercetare

## Faza 1: Semestrul I (OS, Criptografie și Izolare)
>
> Focus: Nivel de memorie, primitive criptografice și apeluri de sistem pe Linux.

- [ ] **1. Timing Leak Detector (Proiect de bază)**
  - **Materii:** Advanced Cryptography (AC) + OS Design and Security (OSDS)
  - **Obiectiv:** Măsurare cu `rdtsc` / `clock_gettime`, analiză statistică (t-test) a scurgerilor de timp.
  - **Edge-cases de testat:**
    - [ ] Optimizările de compilator (`gcc -O3`) care elimină buclele de măsurare.
    - [ ] Poluarea măsurătorilor prin context switching (testat cu `taskset` pe nucleu izolat).

- [ ] **2. Atac Padding Oracle - CCA (Proiect avansat)**
  - **Materii:** Advanced Cryptography (AC)
  - **Obiectiv:** Client-server local AES-CBC; recuperare de text în clar via erori de padding.
  - **Edge-cases de testat:**
    - [ ] Nagle's algorithm și TCP buffering care comasează pachetele.
    - [ ] Tratarea diferențiată a codurilor de eroare vs. latență de răspuns.

- [ ] **3. eBPF/Seccomp Sandbox (Proiect avansat / Cercetare)**
  - **Materii:** OS Design and Security (OSDS) + Practică (Ob.15)
  - **Obiectiv:** Filtrare la nivel de kernel a syscall-urilor periculoase pe procese izolate.
  - **Edge-cases de testat:**
    - [ ] Vulnerabilități TOCTOU pe pointeri din userspace.
    - [ ] Bypass prin syscall-uri de compatibilitate pe 32 de biți (`int 0x80` pe x86_64).

---

## Faza 2: Semestrul II (Rețele, Anomalii și Verificare Formală)
>
> Focus: Logica SMT, analiză de rețea, detecție statistică/ML și bazele disertației.

- [ ] **4. Network Packet Fuzzer + Anomaly Detection (Proiect hibrid)**
  - **Materii:** Network Security (NS) + Cybersecurity (CS) + Curs Opțional (Anomaly Detection)
  - **Obiectiv:** Generare de mutații pe pachete L2/L3 și clasificare trafic anomalic.
  - **Edge-cases de testat:**
    - [ ] Fals-pozitive generate de retransmisii TCP legitime sau ferestre restrânse.
    - [ ] Kernel panic pe manipulare agresivă de raw sockets.

- [ ] **5. Z3 Policy Auditor (Proiect de bază)**
  - **Materii:** Program Verification (PV)
  - **Obiectiv:** Modelare de reguli de acces/firewall ca bit-vectori în Z3; verificare formală a redundanțelor și scurgerilor.
  - **Edge-cases de testat:**
    - [ ] Negații implicite și suprapuneri de măști CIDR.
    - [ ] Verificare rezultate inconsistente (UNSAT fals).

- [ ] **6. Micro-Executor Simbolic (Proiect de cercetare / Sămânță Doctorat)**
  - **Materii:** Program Verification (PV) + Dissertation Research Project
  - **Obiectiv:** Motor de execuție simbolică peste subset x86_64, rezolvare de constrângeri cu Z3.
  - **Edge-cases de testat:**
    - [ ] Path explosion la bucle și salturi condiționate.
    - [ ] Memory aliasing și sincronizarea memoriei liniare cu modelul `Array` din Z3.
