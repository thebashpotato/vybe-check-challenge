--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO postgres;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO postgres;

--
-- Name: trade_fills; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.trade_fills (
    id integer NOT NULL,
    event_timestamp bigint NOT NULL,
    price_in_ticks bigint NOT NULL,
    base_lots_filled bigint NOT NULL
);


ALTER TABLE public.trade_fills OWNER TO postgres;

--
-- Name: trade_fills_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.trade_fills_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.trade_fills_id_seq OWNER TO postgres;

--
-- Name: trade_fills_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.trade_fills_id_seq OWNED BY public.trade_fills.id;


--
-- Name: trade_fills id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.trade_fills ALTER COLUMN id SET DEFAULT nextval('public.trade_fills_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2025-03-02 03:48:12.061495
20250302041052	2025-03-03 06:46:54.169936
\.


--
-- Data for Name: trade_fills; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.trade_fills (id, event_timestamp, price_in_ticks, base_lots_filled) FROM stdin;
1	1740984331	163300	1593
2	1740984331	163350	673
3	1740984341	163170	103
4	1740984350	162890	635
5	1740984363	162904	2424
6	1740984371	162889	2454
7	1740984488	162816	251
8	1740984494	162700	241
9	1740984601	163447	2447
10	1740984608	163254	2450
11	1740984607	163221	24506
12	1740984607	163197	6134
13	1740984620	163257	808
14	1740984607	163288	2449
15	1740984665	162929	2454
16	1740984661	162948	2453
17	1740984675	162713	2458
18	1740984675	162699	1
19	1740984848	161290	4798
20	1740984848	161337	24792
21	1740984853	161444	2477
22	1740984853	161487	619
23	1740984920	161126	107
24	1740984930	161329	66
25	1740984936	161083	2483
26	1740984936	161051	2150
27	1740984954	161427	2477
28	1740984997	160918	2485
29	1740985026	160753	2488
30	1740985086	160649	2489
31	1740985092	160585	1000
32	1740985147	161093	397
33	1740985147	161117	440
34	1740985147	161093	397
35	1740985147	161117	465
36	1740985149	161230	99
37	1740985158	161784	2576
38	1740985201	161778	2472
39	1740985249	162195	1540
40	1740985332	161122	2481
41	1740985349	160926	2485
42	1740985261	161933	2470
43	1740985408	159887	2212
44	1740985408	159945	171
45	1740985502	159969	58
46	1740985604	161659	300
47	1740985564	160836	26
48	1740985607	161654	50
49	1740985626	161781	611
50	1740985684	161154	180
51	1740985828	161335	1
52	1740985828	161303	704
53	1740985853	161360	354
54	1740985955	161915	400
55	1740985933	161322	2478
56	1740985945	161652	47
57	1740986023	162132	38
58	1740986051	162093	2348
59	1740986096	161655	2474
60	1740986096	161687	278
61	1740986124	161966	987
62	1740986124	161966	987
63	1740986214	161838	136
64	1740986259	161753	190
65	1740986259	161773	36903
66	1740986354	162078	699
67	1740986337	161769	2472
68	1740986337	161743	313
69	1740986363	162160	1707
70	1740986437	162306	126
71	1740986341	161932	1404
72	1740986354	162078	699
73	1740986468	161979	100
74	1740986740	160436	177
75	1740986748	160159	19
76	1740986750	160098	191
77	1740986815	160486	2205
78	1740986838	160365	2493
79	1740986934	161683	2473
80	1740986934	161726	3574
81	1740987005	161689	2473
82	1740987067	161995	357
83	1740987353	160903	1900
84	1740987307	161940	84
85	1740987339	161245	2480
86	1740987339	161277	9920
87	1740987339	161311	18576
88	1740987354	160969	350
89	1740987368	161140	2481
90	1740987350	161055	5
91	1740987353	160880	2266
92	1740987353	160923	1644
93	1740987347	161117	2482
94	1740987366	161137	2379
95	1740987640	161316	2479
96	1740987640	161336	7522
97	1740987523	161599	1467
98	1740987646	161224	2480
99	1740987829	161141	620
100	1740987851	161125	260
101	1740987854	161181	268
102	1740987862	161421	917
103	1740987851	161125	260
104	1740987860	161209	2481
105	1740987860	161226	9921
106	1740987942	161727	619
107	1740987984	161931	120
108	1740988067	162305	2161
109	1740987992	162085	400
110	1740988076	162404	1386
111	1740988144	162063	322
\.


--
-- Name: trade_fills_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.trade_fills_id_seq', 111, true);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: trade_fills trade_fills_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.trade_fills
    ADD CONSTRAINT trade_fills_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

