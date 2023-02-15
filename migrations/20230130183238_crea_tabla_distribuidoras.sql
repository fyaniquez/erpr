
-- distribuidoras, tabla de
-- abreviacion EMP

CREATE TABLE IF NOT EXISTS public.distribuidoras (
    id bigint NOT NULL,
    empresa_id BIGINT NOT NULL,
    nombre TEXT NOT NULL,
    descripcion TEXT NOT NULL,
    documento TEXT NOT NULL UNIQUE,
    preventa TEXT NOT NULL,
    activa boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    CONSTRAINT distribuidoras_pkey PRIMARY KEY (id)
);

COMMENT ON COLUMN public.distribuidoras.empresa_id
    IS 'empresa que tiene a esta como distribuidora';
COMMENT ON COLUMN public.distribuidoras.descripcion
    IS 'datos que ayuden a caracterizar a la distribuidora';
COMMENT ON COLUMN public.distribuidoras.preventa
    IS 'dias en los que realiza preventa';

ALTER TABLE public.distribuidoras OWNER TO erp;

CREATE SEQUENCE public.distribuidoras_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.distribuidoras_id_seq OWNER TO erp;

ALTER SEQUENCE public.distribuidoras_id_seq OWNED BY public.distribuidoras.id;

ALTER TABLE ONLY public.distribuidoras ALTER COLUMN id SET DEFAULT nextval('public.distribuidoras_id_seq'::regclass);

-- ajusta ultimo registro
SELECT pg_catalog.setval('public.distribuidoras_id_seq', 1, true);

-- datos
INSERT INTO public.distribuidoras (id, empresa_id, nombre, descripcion, 
    documento, preventa, activa, created_at, updated_at) VALUES (1, 1,
    'EMBOL S.A.', 'Embotelladora Coca Cola', '1007039026', 
    'martes, viernes', true, '2021-03-21 22:06:51.791416', 
    '2021-03-21 22:06:51.791416');

-- constraints
CREATE INDEX index_distribuidoras_on_empresas 
    ON public.distribuidoras USING btree (empresa_id);
ALTER TABLE ONLY public.distribuidoras
    ADD CONSTRAINT fk_dis_emp FOREIGN KEY (empresa_id) 
        REFERENCES public.empresas(id);

ALTER TABLE public.distribuidoras
   ADD CONSTRAINT uk_dis_nombre UNIQUE (empresa_id, nombre);

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION distribuidoras_insert_prepara() 
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
    BEFORE INSERT ON distribuidoras
    FOR EACH ROW
    EXECUTE FUNCTION distribuidoras_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION distribuidoras_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON distribuidoras
    FOR EACH ROW
    EXECUTE FUNCTION distribuidoras_update_prepara();
