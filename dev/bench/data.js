window.BENCHMARK_DATA = {
  "lastUpdate": 1617396174284,
  "repoUrl": "https://github.com/dprint/jsonc-parser",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "f79d24cc3ef3c1ce2517c4db7ff654b9a7cc02fb",
          "message": "chore: Fix benchmark deploy step.",
          "timestamp": "2021-04-02T16:40:29-04:00",
          "tree_id": "d10dced634b4e4e6768ce11ca4f205086354b4fd",
          "url": "https://github.com/dprint/jsonc-parser/commit/f79d24cc3ef3c1ce2517c4db7ff654b9a7cc02fb"
        },
        "date": 1617396172463,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 140988567,
            "range": "± 5677583",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 165642972,
            "range": "± 3980526",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 172275,
            "range": "± 33391",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 214306,
            "range": "± 54631",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 50563,
            "range": "± 10570",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50986,
            "range": "± 5492",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}