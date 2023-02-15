--

CREATE TABLE public.compras (
    id BIGINT NOT NULL,
    fecha timestamp without time zone NOT NULL,
    total integer NOT NULL,
    descuento integer NOT NULL,
    distribuidora_id bigint NOT NULL,
    sucursal_id bigint NOT NULL,
    usuario_id bigint NOT NULL,
    medio_id bigint NOT NULL,
    documento TEXT NOT NULL,
    observaciones TEXT,
    estado text DEFAULT 'Pendiente'::text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.compras OWNER TO erp;

CREATE SEQUENCE public.compras_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.compras_id_seq OWNER TO erp;

ALTER SEQUENCE public.compras_id_seq OWNED BY public.compras.id;

ALTER TABLE ONLY public.compras ALTER COLUMN id SET DEFAULT nextval('public.compras_id_seq'::regclass);

SELECT pg_catalog.setval('public.compras_id_seq', 1, true);

ALTER TABLE ONLY public.compras
    ADD CONSTRAINT compras_pkey PRIMARY KEY (id);

CREATE INDEX index_compras_on_distribuidora_id ON public.compras 
    USING btree (distribuidora_id);
ALTER TABLE ONLY public.compras
    ADD CONSTRAINT fk_compras_distribuidoras
    FOREIGN KEY (distribuidora_id) REFERENCES public.distribuidoras(id);

CREATE INDEX index_compras_on_sucursal_id ON public.compras 
    USING btree (sucursal_id);
ALTER TABLE ONLY public.compras
    ADD CONSTRAINT fk_compras_sucursales
    FOREIGN KEY (sucursal_id) REFERENCES public.sucursales(id);

CREATE INDEX index_compras_on_usuario_id ON public.compras 
    USING btree (usuario_id);
ALTER TABLE ONLY public.compras
    ADD CONSTRAINT fk_compras_usuarios
    FOREIGN KEY (usuario_id) REFERENCES public.usuarios(id);

CREATE INDEX index_compras_on_medio_id ON public.compras 
    USING btree (medio_id);
ALTER TABLE ONLY public.compras
    ADD CONSTRAINT fk_compras_medios
    FOREIGN KEY (medio_id) REFERENCES public.medios(id);

ALTER TABLE public.compras ADD CONSTRAINT chk_estado 
    CHECK ((estado = ANY (
        ARRAY['Anulado'::text, 'Pagado'::text, 'Pendiente'::text])));

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION compras_insert_prepara() 
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
    BEFORE INSERT ON compras
    FOR EACH ROW
    EXECUTE FUNCTION compras_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION compras_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON compras
    FOR EACH ROW
    EXECUTE FUNCTION compras_update_prepara();
