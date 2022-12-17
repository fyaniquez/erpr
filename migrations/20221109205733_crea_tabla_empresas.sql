-- empresas, tabla de
-- abreviacion EMP

CREATE TABLE IF NOT EXISTS public.empresas (
    id bigint NOT NULL,
    nombre TEXT NOT NULL UNIQUE,
    nit TEXT NOT NULL UNIQUE,
    activa boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    CONSTRAINT empresas_pkey PRIMARY KEY (id)
);


ALTER TABLE public.empresas OWNER TO erp;

CREATE SEQUENCE public.empresas_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.empresas_id_seq OWNER TO erp;

ALTER SEQUENCE public.empresas_id_seq OWNED BY public.empresas.id;

ALTER TABLE ONLY public.empresas ALTER COLUMN id SET DEFAULT nextval('public.empresas_id_seq'::regclass);

-- datos

INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (1, 'Yañiquez Monasterios, Favio Jorge', '2710988016', true, '2021-03-21 22:06:51.791416', '2021-03-21 22:06:51.791416');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (3, 'Nordix S.R.L.', '381217020', true, '2021-04-04 00:00:00', '2021-04-04 00:00:00');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (4, 'Molino Andino S.A.', '1020387020', true, '2021-04-05 02:12:35.893467', '2021-04-05 02:12:35.893467');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (5, 'Operaciones del Pacífico Ltda.', '1015067025', true, '2021-04-05 03:05:35.026278', '2021-04-05 03:05:35.026278');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (6, 'PIL Andina S.A.', '1020757027', true, '2021-04-05 13:14:44.303213', '2021-04-05 13:14:44.303213');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (7, 'STEGE', '1020477022', true, '2021-04-05 13:18:19.093176', '2021-04-05 13:18:19.093176');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (8, 'Sofia', '1028779020', true, '2021-04-05 13:20:39.198002', '2021-04-05 13:20:39.198002');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (9, 'Distribuidora Fenix', '', true, '2021-04-05 13:22:20.297632', '2021-04-05 13:22:20.297632');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (12, 'Viodek', '123456789', true, '2021-10-07 19:02:32.62018', '2021-10-07 19:02:32.620187');
INSERT INTO public.empresas (id, nombre, nit, activa, created_at, updated_at) VALUES (13, 'Delizia', '1020493029', true, '2021-10-09 23:09:03.165446', '2021-10-09 23:09:03.165475');

-- ajusta ultimo registro
SELECT pg_catalog.setval('public.empresas_id_seq', 13, true);

-- constraints
-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION empresas_insert_prepara() 
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
    BEFORE INSERT ON empresas
    FOR EACH ROW
    EXECUTE FUNCTION empresas_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION empresas_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON empresas
    FOR EACH ROW
    EXECUTE FUNCTION empresas_update_prepara();
