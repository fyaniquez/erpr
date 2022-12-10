-- paises, tabla de 
-- abreviacion PAI
CREATE TABLE IF NOT EXISTS public.paises
(
    id bigint NOT NULL,
    sigla TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT paises_pkey PRIMARY KEY (id)
);

ALTER TABLE public.paises OWNER TO erp;
CREATE SEQUENCE public.paises_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER TABLE public.paises_id_seq OWNER TO erp;
ALTER SEQUENCE public.paises_id_seq OWNED BY public.paises.id;
ALTER TABLE ONLY public.paises ALTER COLUMN id SET DEFAULT nextval('public.paises_id_seq'::regclass);

-- datos
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (1, 'DE', 'Alemania', '2021-03-21 16:57:45.41259', '2021-03-21 16:57:45.41259');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (2, 'AR', 'Argentina', '2021-03-21 16:57:45.50073', '2021-03-21 16:57:45.50073');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (3, 'BD', 'Bangladesh', '2021-03-21 16:57:45.519585', '2021-03-21 16:57:45.519585');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (4, 'BO', 'Bolivia', '2021-03-21 16:57:45.536038', '2021-03-21 16:57:45.536038');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (5, 'BR', 'Brasil', '2021-03-21 16:57:45.552714', '2021-03-21 16:57:45.552714');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (6, 'CL', 'Chile', '2021-03-21 16:57:45.569356', '2021-03-21 16:57:45.569356');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (7, 'CN', 'China', '2021-03-21 16:57:45.586005', '2021-03-21 16:57:45.586005');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (8, 'CO', 'Colombia', '2021-03-21 16:57:45.602716', '2021-03-21 16:57:45.602716');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (9, 'KO', 'Corea', '2021-03-21 16:57:45.619394', '2021-03-21 16:57:45.619394');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (10, 'EC', 'Ecuador', '2021-03-21 16:57:45.636018', '2021-03-21 16:57:45.636018');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (11, 'US', 'EE.UU.', '2021-03-21 16:57:45.652704', '2021-03-21 16:57:45.652704');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (12, 'MX', 'Mexico', '2021-03-21 16:57:45.669413', '2021-03-21 16:57:45.669413');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (13, 'NL', 'Paises Bajos', '2021-03-21 16:57:45.686078', '2021-03-21 16:57:45.686078');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (14, 'PE', 'Peru', '2021-03-21 16:57:45.702692', '2021-03-21 16:57:45.702692');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (15, 'TR', 'Turquia', '2021-03-21 16:57:45.719385', '2021-03-21 16:57:45.719385');
INSERT INTO public.paises (id, sigla, nombre, created_at, updated_at) VALUES (16, 'JP', 'Japón', '2021-10-11 20:00:00', '2021-10-11 20:00:00');
SELECT pg_catalog.setval('public.paises_id_seq', 16, true);

-- constraints
-- trigger para preparar el insert
CREATE OR REPLACE FUNCTION paises_insert_prepara() 
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
    BEFORE INSERT ON paises
    FOR EACH ROW
    EXECUTE FUNCTION paises_insert_prepara();

-- trigger para preparar el update 
CREATE OR REPLACE FUNCTION paises_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON paises
    FOR EACH ROW
    EXECUTE FUNCTION paises_update_prepara();

