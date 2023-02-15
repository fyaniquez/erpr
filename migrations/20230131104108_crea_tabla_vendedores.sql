--

CREATE TABLE public.vendedores (
    id bigint NOT NULL,
    distribuidora_id BIGINT NOT NULL,
    cargo text NOT NULL,
    nombre text NOT NULL,
    contactos text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.vendedores OWNER TO erp;

COMMENT ON TABLE public.vendedores IS 'registro de vendedores y otros empleados de los distribuidores';


CREATE SEQUENCE public.vendedores_id_seq
    AS bigint
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.vendedores_id_seq OWNER TO erp;

ALTER SEQUENCE public.vendedores_id_seq OWNED BY public.vendedores.id;

ALTER TABLE ONLY public.vendedores ALTER COLUMN id SET DEFAULT nextval('public.vendedores_id_seq'::regclass);

SELECT pg_catalog.setval('public.vendedores_id_seq', 1, true);

ALTER TABLE ONLY public.vendedores
    ADD CONSTRAINT vendedores_pkey PRIMARY KEY (id);
 
ALTER TABLE ONLY public.vendedores
    ADD CONSTRAINT uk_ven_nombre UNIQUE (distribuidora_id, nombre);

CREATE INDEX index_vendedores_on_distribuidora
    ON public.vendedores USING btree (distribuidora_id);
ALTER TABLE ONLY public.vendedores
    ADD CONSTRAINT fk_ven_dis FOREIGN KEY (distribuidora_id)
    REFERENCES public.distribuidoras(id) 
    ON UPDATE RESTRICT ON DELETE RESTRICT;

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION vendedores_insert_prepara() 
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
    BEFORE INSERT ON vendedores
    FOR EACH ROW
    EXECUTE FUNCTION vendedores_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION vendedores_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON vendedores
    FOR EACH ROW
    EXECUTE FUNCTION vendedores_update_prepara();
