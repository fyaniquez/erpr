
--

CREATE TABLE public.comprados (
    id bigint NOT NULL,
    cantidad integer NOT NULL,
    costo integer NOT NULL,
    descuento integer NOT NULL,
    total integer NOT NULL,
    vencimiento timestamp without time zone NOT NULL,
    producto_id bigint NOT NULL,
    compra_id bigint NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

ALTER TABLE public.comprados OWNER TO erp;

CREATE SEQUENCE public.comprados_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.comprados_id_seq OWNER TO erp;

ALTER SEQUENCE public.comprados_id_seq OWNED BY public.comprados.id;

ALTER TABLE ONLY public.comprados ALTER COLUMN id 
    SET DEFAULT nextval('public.comprados_id_seq'::regclass);

SELECT pg_catalog.setval('public.comprados_id_seq', 1, true);

ALTER TABLE ONLY public.comprados
    ADD CONSTRAINT comprados_pkey PRIMARY KEY (id);

CREATE INDEX index_comprados_on_compra_id ON public.comprados 
    USING btree (compra_id);

ALTER TABLE ONLY public.comprados
    ADD CONSTRAINT fk_comprados_compra
    FOREIGN KEY (compra_id) REFERENCES public.compras(id);

CREATE INDEX fki_fk_comprados_producto ON public.comprados 
    USING btree (producto_id);

ALTER TABLE ONLY public.comprados
    ADD CONSTRAINT fk_comprados_producto 
    FOREIGN KEY (producto_id) REFERENCES public.productos(id) 
    ON UPDATE RESTRICT ON DELETE RESTRICT;
-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION comprados_insert_prepara() 
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
    BEFORE INSERT ON comprados
    FOR EACH ROW
    EXECUTE FUNCTION comprados_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION comprados_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON comprados
    FOR EACH ROW
    EXECUTE FUNCTION comprados_update_prepara();
