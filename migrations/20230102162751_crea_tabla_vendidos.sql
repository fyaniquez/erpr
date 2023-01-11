--

CREATE TABLE public.vendidos (
    id bigint NOT NULL,
    cantidad integer NOT NULL,
    precio integer NOT NULL,
    descuento integer NOT NULL,
    total integer NOT NULL,
    producto_id bigint NOT NULL,
    venta_id bigint NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

ALTER TABLE public.vendidos OWNER TO erp;

CREATE SEQUENCE public.vendidos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.vendidos_id_seq OWNER TO erp;

ALTER SEQUENCE public.vendidos_id_seq OWNED BY public.vendidos.id;

ALTER TABLE ONLY public.vendidos ALTER COLUMN id 
    SET DEFAULT nextval('public.vendidos_id_seq'::regclass);

INSERT INTO public.vendidos (id, cantidad, precio, descuento, total, producto_id, venta_id, created_at, updated_at) VALUES (1, 100, 100, 100, 0, 724, 1, '2022-07-23 15:52:39.846127', '2022-07-23 15:52:39.846129');
INSERT INTO public.vendidos (id, cantidad, precio, descuento, total, producto_id, venta_id, created_at, updated_at) VALUES (2, 100, 2700, 2700, 0, 815, 1, '2022-07-23 15:53:02.288882', '2022-07-23 15:53:02.288885');
INSERT INTO public.vendidos (id, cantidad, precio, descuento, total, producto_id, venta_id, created_at, updated_at) VALUES (3, 100, 500, 500, 0, 798, 2, '2022-08-15 22:14:58.049783', '2022-08-15 22:14:58.049787');
INSERT INTO public.vendidos (id, cantidad, precio, descuento, total, producto_id, venta_id, created_at, updated_at) VALUES (4, 100, 300, 300, 0, 1028, 2, '2022-08-15 22:14:58.123405', '2022-08-15 22:14:58.123407');

SELECT pg_catalog.setval('public.vendidos_id_seq', 4, true);

ALTER TABLE ONLY public.vendidos
    ADD CONSTRAINT vendidos_pkey PRIMARY KEY (id);

CREATE INDEX index_vendidos_on_venta_id ON public.vendidos 
    USING btree (venta_id);

ALTER TABLE ONLY public.vendidos
    ADD CONSTRAINT fk_vendidos_venta
    FOREIGN KEY (venta_id) REFERENCES public.ventas(id);

CREATE INDEX fki_fk_vendidos_producto ON public.vendidos 
    USING btree (producto_id);

ALTER TABLE ONLY public.vendidos
    ADD CONSTRAINT fk_vendidos_producto 
    FOREIGN KEY (producto_id) REFERENCES public.productos(id) 
    ON UPDATE RESTRICT ON DELETE RESTRICT;

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION vendidos_insert_prepara() 
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
    BEFORE INSERT ON vendidos
    FOR EACH ROW
    EXECUTE FUNCTION vendidos_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION vendidos_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON vendidos
    FOR EACH ROW
    EXECUTE FUNCTION vendidos_update_prepara();
