--

CREATE TABLE public.ventas (
    id bigint NOT NULL,
    fecha timestamp without time zone NOT NULL,
    total integer NOT NULL,
    descuento integer NOT NULL,
    cliente_id bigint NOT NULL,
    puesto_id bigint NOT NULL,
    usuario_id bigint NOT NULL,
    medio_id bigint NOT NULL,
    estado text DEFAULT 'Pendiente'::text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.ventas OWNER TO erp;

CREATE SEQUENCE public.ventas_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.ventas_id_seq OWNER TO erp;

ALTER SEQUENCE public.ventas_id_seq OWNED BY public.ventas.id;

ALTER TABLE ONLY public.ventas ALTER COLUMN id SET DEFAULT nextval('public.ventas_id_seq'::regclass);

INSERT INTO public.ventas (id, fecha, total, descuento, cliente_id, puesto_id, usuario_id, medio_id, estado, created_at, updated_at) VALUES (1, '2022-07-23 15:52:33.951947', 2800, 0, 1, 1, 1, 1, 'Pagado','2022-07-23 15:52:33.95195', '2022-07-23 15:52:33.95195');
INSERT INTO public.ventas (id, fecha, total, descuento, cliente_id, puesto_id, usuario_id, medio_id, estado, created_at, updated_at) VALUES (2, '2022-07-23 15:52:33.951947', 700, 100, 1, 1, 1, 1, 'Pagado','2022-07-23 15:52:33.95195', '2022-07-23 15:52:33.95195');

SELECT pg_catalog.setval('public.ventas_id_seq', 2, true);

ALTER TABLE ONLY public.ventas
    ADD CONSTRAINT ventas_pkey PRIMARY KEY (id);

CREATE INDEX index_ventas_on_cliente_id ON public.ventas 
    USING btree (cliente_id);
ALTER TABLE ONLY public.ventas
    ADD CONSTRAINT fk_ventas_clientes
    FOREIGN KEY (cliente_id) REFERENCES public.clientes(id);

CREATE INDEX index_ventas_on_puesto_id ON public.ventas 
    USING btree (puesto_id);
ALTER TABLE ONLY public.ventas
    ADD CONSTRAINT fk_ventas_puestos
    FOREIGN KEY (puesto_id) REFERENCES public.puestos(id);

CREATE INDEX index_ventas_on_usuario_id ON public.ventas 
    USING btree (usuario_id);
ALTER TABLE ONLY public.ventas
    ADD CONSTRAINT fk_ventas_usuarios
    FOREIGN KEY (usuario_id) REFERENCES public.usuarios(id);

CREATE INDEX index_ventas_on_medio_id ON public.ventas 
    USING btree (medio_id);
ALTER TABLE ONLY public.ventas
    ADD CONSTRAINT fk_ventas_medios
    FOREIGN KEY (medio_id) REFERENCES public.medios(id);

ALTER TABLE public.ventas ADD CONSTRAINT chk_estado 
    CHECK ((estado = ANY (
        ARRAY['Anulado'::text, 'Pagado'::text, 'Pendiente'::text])));

-- Completed on 2022-12-29 19:24:52 -04

--
-- PostgreSQL database dump complete
--

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION ventas_insert_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.created_at = now();
	NEW.updated_at = now();
    NEW.fecha = now();
    NEW.estado = 'Pagado';
    RETURN NEW;
END;
$$;

CREATE TRIGGER insert_prepara
    BEFORE INSERT ON ventas
    FOR EACH ROW
    EXECUTE FUNCTION ventas_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION ventas_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON ventas
    FOR EACH ROW
    EXECUTE FUNCTION ventas_update_prepara();
