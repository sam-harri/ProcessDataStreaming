-- Set verbosity to show notices
SET client_min_messages TO NOTICE;

\echo '--- Starting Database Initialization ---';

-- Processing "distillation_column_data" table
\echo 'Processing "distillation_column_data" table...';

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT FROM information_schema.tables 
        WHERE table_schema = 'public' 
        AND table_name = 'distillation_column_data'
    ) THEN
        CREATE TABLE distillation_column_data (
            uuid VARCHAR NOT NULL,
            timestamp BIGINT NOT NULL,
            temperature FLOAT NOT NULL,
            pressure FLOAT NOT NULL,
            reflux_ratio FLOAT NOT NULL,
            bottom_flow_rate FLOAT NOT NULL,
            top_flow_rate FLOAT NOT NULL,
            PRIMARY KEY (uuid, timestamp)
        );
        RAISE NOTICE 'Table "distillation_column_data" has been created.';
    ELSE
        RAISE NOTICE 'Table "distillation_column_data" already exists.';
    END IF;
END;
$$;

\echo 'Done with "distillation_column_data" table.';

-- Processing "heat_exchanger_data" table
\echo 'Processing "heat_exchanger_data" table...';

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT FROM information_schema.tables 
        WHERE table_schema = 'public' 
        AND table_name = 'heat_exchanger_data'
    ) THEN
        CREATE TABLE heat_exchanger_data (
            uuid VARCHAR NOT NULL,
            timestamp BIGINT NOT NULL,
            tc_in FLOAT NOT NULL,
            tc_out FLOAT NOT NULL,
            th_in FLOAT NOT NULL,
            th_out FLOAT NOT NULL,
            flow_rate FLOAT NOT NULL,
            pressure_drop FLOAT NOT NULL,
            PRIMARY KEY (uuid, timestamp)
        );
        RAISE NOTICE 'Table "heat_exchanger_data" has been created.';
    ELSE
        RAISE NOTICE 'Table "heat_exchanger_data" already exists.';
    END IF;
END;
$$;

\echo 'Done with "heat_exchanger_data" table.';

\echo '--- Database Initialization Complete ---';
