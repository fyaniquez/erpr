-- Add migration script here

CREATE TABLE IF NOT EXISTS public.usuarios
(
    id bigint NOT NULL,
    nombre TEXT NOT NULL,
    documento TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_digest TEXT,
    estado TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT usuarios_pkey PRIMARY KEY (id)
);

ALTER TABLE public.usuarios OWNER TO erp;
CREATE SEQUENCE public.usuarios_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER TABLE public.usuarios_id_seq OWNER TO erp;
ALTER SEQUENCE public.usuarios_id_seq OWNED BY public.usuarios.id;
ALTER TABLE ONLY public.usuarios ALTER COLUMN id SET DEFAULT nextval('public.usuarios_id_seq'::regclass);

-- datos

INSERT INTO usuarios (id, nombre, documento, email, estado, created_at, updated_at) 
VALUES (1, 'NN', 'NN', 'NN', 'activo', '2021-12-01 23:09:35.770143', '2021-12-01 23:09:35.770143');
INSERT INTO usuarios (id, nombre, documento, email, estado, created_at, updated_at) 
VALUES (2, 'Favio Yañiquez', '2710988016', 'fyaniquez@gmail.com', 'pendiente', '2021-12-01 23:09:35.770143', '2021-12-01 23:09:35.770143');

SELECT pg_catalog.setval('public.usuarios_id_seq', 3, true);

-- constraints
-- trigger para insert
CREATE OR REPLACE FUNCTION usuarios_insert_prepara() 
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
    BEFORE INSERT ON usuarios
    FOR EACH ROW
    EXECUTE FUNCTION usuarios_insert_prepara();

-- trigger para update
CREATE OR REPLACE FUNCTION usuarios_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON usuarios
    FOR EACH ROW
    EXECUTE FUNCTION usuarios_update_prepara();
