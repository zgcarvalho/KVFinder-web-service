# KVFinder-web service

![GitHub release (latest by date)](https://img.shields.io/github/v/release/LBC-LNBio/KVFinder-web-service?color=informational)
![GitHub](https://img.shields.io/github/license/LBC-LNBio/KVFinder-web-service)

Welcome to the KVFinder-web service, this page was built to help you get started with our cavity detection web service.

## KVFinder-web

KVFinder-web is an open-source web-based application of an updated version of [parKVFinder](https://github.com/LBC-LNBio/parKVFinder) software (v1.2.0) cavity detection and characterization of any type of biomolecular structure. The characterization includes spatial, depth, constitutional and hydropathy characterization.

The KVFinder-web has two independent components:

- a RESTful web service: [KVFinder-web service](https://github.com/LBC-LNBio/KVFinder-web-service);
- a graphical web portal: [KVFinder-web portal](https://github.com/LBC-LNBio/KVFinder-web-portal).

To broaden the range of possibilities for user interaction, we also provide additional client-side applications, that are:

- a graphical PyMOL plugin: [PyMOL KVFinder-web Tools](https://github.com/LBC-LNBio/PyMOL-KVFinder-web-Tools);
- an example of a Python HTTP client: [http-client.py](https://github.com/LBC-LNBio/KVFinder-web-service/blob/master/http_client.py).

The full KVFinder-web documentation can be found here: [http://lbc-lnbio.github.io/KVFinder-web](http://lbc-lnbio.github.io/KVFinder-web).

### KVFinder-web service

KVFinder-web service is a RESTful web service that runs an updated version of [parKVFinder](https://github.com/LBC-LNBio/parKVFinder) software (v1.2.0) to detect and characterize cavities. KVFinder-web service has three modules: web, queue and worker. Each one runs in single docker containers, but they are combined into a docker-compose configuration file.

We provide a publicly available KVFinder-web service ([https://kvfinder-web.cnpem.br](https://kvfinder-web.cnpem.br)), with [KVFinder-web portal](https://github.com/LBC-LNBio/KVFinder-web-portal) as the graphical web interface.

Our public KVFinder-web service is hosted in a cloud environment, that has some limitations compared to parKVFinder standalone version, which are stated on the documentation. Hence, users may opt to run jobs on our public KVFinder-web service or on a locally configured server.

#### Local installation

To run this web service in Linux distributions, it is necessary to install docker-compose and its dependencies. To install it:

```bash
sudo apt install docker-compose
```

After the docker-compose installation and clone of this repository. To start KVFinder-web service, you can execute the command bellow at the root of KVFinder-web-service repository (where `docker-compose.yml` file is located):

```bash
docker-compose up
```

The KVFinder-web service uses port 8081 by default. If the local installation was successfully, “KVFinder-web service” message will be shown at [http://localhost:8081](http://localhost:8081) and Job queue information can be accessed at [http://localhost:8023/info](http://localhost:8023/info).

#### API

To create a job:

- POST /create
  - Method: POST
  - Media type: 'application/json'
  - URL: [http://localthost:8081/create](http://localthost:8081/create)

Example of job request:

```json
{
  "pdb": "MODEL        1\nATOM      1  N   GLU E  13      -6.693 -15.642 -14.858  1.00100.00           N  \n(...)\nEND\n",
  "settings": {
    "modes": {
      "whole_protein_mode": true,
      "box_mode": false,
      "resolution_mode": "Low",
      "surface_mode": true,
      "kvp_mode": false,
      "ligand_mode": false
    },
    "step_size": {
      "step_size": 0.0
    },
    "probes": {
      "probe_in": 1.4,
      "probe_out": 4.0
    },
    "cutoffs": {
      "volume_cutoff": 5.0,
      "ligand_cutoff": 5.0,
      "removal_distance": 0.0
    },
    "visiblebox": {
      "p1": { "x": 0.0, "y": 0.0, "z": 0.0 },
      "p2": { "x": 0.0, "y": 0.0, "z": 0.0 },
      "p3": { "x": 0.0, "y": 0.0, "z": 0.0 },
      "p4": { "x": 0.0, "y": 0.0, "z": 0.0 }
    },
    "internalbox": {
      "p1": { "x": -4.0, "y": -4.0, "z": -4.0 },
      "p2": { "x": 4.0, "y": -4.0, "z": -4.0 },
      "p3": { "x": -4.0, "y": 4.0, "z": -4.0 },
      "p4": { "x": -4.0, "y": -4.0, "z": 4.0 }
    }
  }
}
```

To request a job:

- GET /:id
  - Method: GET
  - URL: [http://localhost:8081/:id](http://localhost:8081/:id)

Where *:id* is the job id received from the KVFinder-web service as submission response.

Example of response obtained for a *job* with status "queued":

```json
  {
    "id": "4990580026958948484",
    "status": "queued",
    "output": null,
    "created_at": "2023-03-03T18:55:28.439300871Z",
    "started_at": null,
    "ended_at": null,
    "expires_after": "1day"
  }
```

Example of response obtained for a *job* with status "running":

```json
  {
    "id": "4990580026958948484",  
    "status": "running",  
    "output": null,  
    "created_at": "2023-03-03T18:55:28.439300871Z",  
    "started_at": "2023-03-03T18:55:31.416200437Z",    
    "ended_at": null,  
    "expires_after": "1day"
  }
```

Example of response obtained for a *job* with status "completed":

```json
{
  "id": "4990580026958948484",
  "status": "completed",
  "output": {
    "pdb_kv": "ATOM      1  HS  KAA   259     -15.000 -10.200   0.000  1.00  0.00\nATOM      2(...)",
    "report": "# TOML results file for parKVFinder software\n\ntitle = \"parKVFinder results f(...)",
    "log": "==========\tSTART\tRUN\t=========\n\nDate and time: Fri Apr 16 11:40:06 2021\n\nRu(...)",
  },
  "created_at": "2021-04-16T11:40:02.514045822Z",
  "started_at": "2021-04-16T11:40:06.671064517Z",
  "ended_at": "2021-04-16T11:40:17.701426882Z",
  "expires_after": "1day"
}
```

To retrieve a job input:

- GET /retrieve-input/:id*
  - Method: GET
  - URL: [http://localhost:8081/retrieve-input/:id](http://localhost:8081/retrieve-input/:id)

Where *:id*  is the job id received from the server as submission response.

Example of response obtained for a requested *job* input:

```json
{
  "id": "4990580026958948484",
  "input": {
    "pdb": "ATOM   25  OD1 ASP E 323       0.497  12.598  16.506  1.00 40.80           O  \nATOM      26(...)",
    "pdb_ligand": null,
    "settings": {"probes": (...)},
  },
  "created_at": "2022-01-25T19:32:13.572099997Z",
}
```

### HTTP Client

In this repository, we provide a simple [Python HTTP client](https://github.com/LBC-LNBio/KVFinder-web-service/blob/master/http-client.py) to interact with KVFinder-web service via `requests` package. This client provides an example of a template in Python that can be run to access our web service and parse the output data.

### KVFinder-web portal

The KVFinder-web portal, written in R and Shiny, is a graphical web application for detecting and characterizing biomolecular cavities at a KVFinder-web service, natively configured in our publicly available web service ([http://kvfinder-web.cnpem.br](http://kvfinder-web.cnpem.br)).

### PyMOL KVFinder-web Tools

The PyMOL KVFinder-web Tools, written in Python and Qt, is a PyMOL v2.x plugin for detecting and characterizing biomolecular cavities at a KVFinder-web service with functionalities similar to [PyMOL parKVFinder Tools](https://github.com/LBC-LNBio/parKVFinder/wiki/parKVFinder-Tutorial#pymol2-parkvfinder-tools), which is natively configured to our publicly available web service ([http://kvfinder-web.cnpem.br](http://kvfinder-web.cnpem.br)).

## Funding

KVFinder-web interface was supported by Fundação de Amparo à Pesquisa do Estado de São Paulo (FAPESP) [Grant Number 2018/00629-0], Brazilian Biosciences National Laboratory (LNBio) and Brazilian Center for Research in Energy and Materials (CNPEM).

## License

The software is licensed under the terms of the Apache-2.0 License and is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the Apache-2.0 License for more details.

---
