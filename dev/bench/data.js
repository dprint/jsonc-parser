window.BENCHMARK_DATA = {
  "lastUpdate": 1617406128984,
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
      },
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
          "id": "07bbde4620374dd37ddc8334162d833ca98e2e80",
          "message": "chore(scanner): Use a char iterator instead of allocating a vector of chars.",
          "timestamp": "2021-04-02T17:47:05-04:00",
          "tree_id": "539c25ee9488fbc87ef0978e81ac1fa4c60ad39d",
          "url": "https://github.com/dprint/jsonc-parser/commit/07bbde4620374dd37ddc8334162d833ca98e2e80"
        },
        "date": 1617400161835,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 134697670,
            "range": "± 7009667",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 160727909,
            "range": "± 7575121",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 212512,
            "range": "± 9091",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 255581,
            "range": "± 8492",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 70738,
            "range": "± 1414",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 71150,
            "range": "± 3672",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "84f3b537355e866dc5389d822ed10c95c9df6b5f",
          "message": "perf: Comment text references original string (part of #10)",
          "timestamp": "2021-04-02T18:06:06-04:00",
          "tree_id": "1663de640af21a1a315843595158fd19dd31b57d",
          "url": "https://github.com/dprint/jsonc-parser/commit/84f3b537355e866dc5389d822ed10c95c9df6b5f"
        },
        "date": 1617401316667,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 123925235,
            "range": "± 8943739",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 154665110,
            "range": "± 17871619",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 178589,
            "range": "± 19557",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 216027,
            "range": "± 20409",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 43346,
            "range": "± 5490",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 42185,
            "range": "± 6088",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "620807aa79d639cd69d6147afa2ab4a474c439a4",
          "message": "perf: Reference original string for word and number literals (More work for #10)",
          "timestamp": "2021-04-02T18:45:06-04:00",
          "tree_id": "d909147c722d2e11006507d403e96eec16d16821",
          "url": "https://github.com/dprint/jsonc-parser/commit/620807aa79d639cd69d6147afa2ab4a474c439a4"
        },
        "date": 1617403665573,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 140116759,
            "range": "± 18228302",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 162037660,
            "range": "± 12535190",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 204224,
            "range": "± 39735",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 242315,
            "range": "± 40806",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 48086,
            "range": "± 7287",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50842,
            "range": "± 7743",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "c2777030c13adf4162fefed691d8fefe127d8266",
          "message": "perf: #10 - Reference the original string for string literals more often.",
          "timestamp": "2021-04-02T19:26:21-04:00",
          "tree_id": "6cfb8f70bd28056c715fef8fdaa959786fd7f155",
          "url": "https://github.com/dprint/jsonc-parser/commit/c2777030c13adf4162fefed691d8fefe127d8266"
        },
        "date": 1617406128515,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 124763267,
            "range": "± 6681993",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 161634298,
            "range": "± 13965490",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 135194,
            "range": "± 15942",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 183321,
            "range": "± 30977",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 49489,
            "range": "± 6299",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50987,
            "range": "± 5197",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}