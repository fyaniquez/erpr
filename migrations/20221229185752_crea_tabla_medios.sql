--

CREATE TABLE public.medios (
    id bigint NOT NULL,
    nombre text NOT NULL,
    sigla text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.medios OWNER TO erp;

COMMENT ON TABLE public.medios IS 'medios de pago habilitados';

CREATE SEQUENCE public.medios_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.medios_id_seq OWNER TO erp;

ALTER SEQUENCE public.medios_id_seq OWNED BY public.medios.id;

ALTER TABLE ONLY public.medios ALTER COLUMN id 
    SET DEFAULT nextval('public.medios_id_seq'::regclass);

INSERT INTO public.medios (id, nombre, sigla, created_at, updated_at) VALUES (1, 'efectivo', 'EFE', '2022-12-29 18:00:00', '2022-12-29 18:00:00');
INSERT INTO public.medios (id, nombre, sigla, created_at, updated_at) VALUES (2, 'QR', 'QR', '2022-12-29 18:00:00', '2022-12-29 18:00:00');
INSERT INTO public.medios (id, nombre, sigla, created_at, updated_at) VALUES (3, 'crédito', 'CRE', '2022-12-29 18:00:00', '2022-12-29 18:00:00');
INSERT INTO public.medios (id, nombre, sigla, created_at, updated_at) VALUES (4, 'tarjeta crédito', 'TC', '2022-12-29 18:00:00', '2022-12-29 18:00:00');
INSERT INTO public.medios (id, nombre, sigla, created_at, updated_at) VALUES (5, 'tigo money', 'TM', '2022-12-29 18:00:00', '2022-12-29 18:00:00');


SELECT pg_catalog.setval('public.medios_id_seq', 5, true);

ALTER TABLE ONLY public.medios
    ADD CONSTRAINT medios_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.medios
    ADD CONSTRAINT uk_medios_nombre UNIQUE (nombre);

ALTER TABLE ONLY public.medios
    ADD CONSTRAINT uk_medios_sigla UNIQUE (sigla);


-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION medios_insert_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.created_at = now();
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER insert_prepara
    BEFORE INSERT ON medios
    FOR EACH ROW
    EXECUTE FUNCTION medios_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION medios_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON medios
    FOR EACH ROW
    EXECUTE FUNCTION medios_update_prepara();
