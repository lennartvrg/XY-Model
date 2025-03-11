BEGIN;

CREATE TABLE IF NOT EXISTS "runs" (
    id                  INTEGER     NOT NULL,
    created_at          INTEGER     NOT NULL,

    CONSTRAINT "PK.Runs_ID" PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS "results" (
    id                  INTEGER     NOT NULL,
    run_id              INTEGER     NOT NULL,
    dimension           INTEGER     NOT NULL,

    size                INTEGER     NOT NULL,
    temperature         REAL        NOT NULL,

    energy              REAL        NOT NULL,
    energy_std          REAL        NOT NULL,
    energy_tau          REAL        NOT NULL,

    energy_sqr          REAL        NOT NULL,
    energy_sqr_std      REAL        NOT NULL,
    energy_sqr_tau      REAL        NOT NULL,

    magnet              REAL        NOT NULL,
    magnet_std          REAL        NOT NULL,
    magnet_tau          REAL        NOT NULL,

    magnet_sqr          REAL        NOT NULL,
    magnet_sqr_std      REAL        NOT NULL,
    magnet_sqr_tau      REAL        NOT NULL,

    specific_heat       REAL        NOT NULL,
    specific_heat_std   REAL        NOT NULL,
    magnet_suscept      REAL        NOT NULL,
    magnet_suscept_std  REAL        NOT NULL,

    time_mc             INTEGER     NOT NULL,
    time_boot           INTEGER     NOT NULL,

    CONSTRAINT "PK.Results_ID" PRIMARY KEY (id),
    CONSTRAINT "FK.Results_RunID" FOREIGN KEY (run_id) REFERENCES "runs" (id)
);

CREATE INDEX IF NOT EXISTS "IX.Results_RunID_Dimension" ON "results" (run_id, dimension);

COMMIT;