CREATE TABLE IF NOT EXISTS "runs" (
    id                  INTEGER     NOT NULL,
    created_at          INTEGER     NOT NULL,

    CONSTRAINT "PK.Runs_ID" PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS "configurations" (
    id                  INTEGER     NOT NULL,
    run_id              INTEGER     NOT NULL,

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
    magnet_suscept      REAL        NOT NULL,

    CONSTRAINT "PK.Configurations_ID" PRIMARY KEY (id),
    CONSTRAINT "FK.Configurations_RunID" FOREIGN KEY (run_id) REFERENCES "runs" (id)
);

CREATE INDEX "IX.Configurations_RunID" ON "configurations" (run_id);

CREATE TABLE IF NOT EXISTS "observables" (
    configuration_id    INTEGER     NOT NULL,
    sequence_id         INTEGER     NOT NULL,

    e                   REAL        NOT NULL,
    m                   REAL        NOT NULL,

    CONSTRAINT "PK.Observables_ConfigurationID_SequenceID" PRIMARY KEY (configuration_id, sequence_id),
    CONSTRAINT "FK.Observables_ConfigurationID" FOREIGN KEY (configuration_id) REFERENCES "configurations" (id)
);
