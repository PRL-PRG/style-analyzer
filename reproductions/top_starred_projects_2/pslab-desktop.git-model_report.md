# Model report for file:///tmp/top-repos-quality-repos-1r838adt/pslab-desktop.git HEAD 669f850676797920d012ad3047f6cc340b52c695

### Dump

```json
{'created_at': '2021-08-30 07:20:30',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '22.2 kB',
 'tags': [],
 'uuid': '75087005-f7c1-4bb4-b65f-cd556e0c86c8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1r838adt/pslab-desktop.git 669f850676797920d012ad3047f6cc340b52c695

# javascript
234 rules, avg.len. 7.4
## train
PPCR: 0.990990
### report
macro
{'f1-score': 0.7771025811917275,
 'precision': 0.8299850421272025,
 'recall': 0.7396003660325476,
 'support': 46743}
micro
{'f1-score': 0.9386004321502684,
 'precision': 0.9386004321502684,
 'recall': 0.9386004321502684,
 'support': 46743}
weighted
{'f1-score': 0.9355935126110413,
 'precision': 0.9363605262175394,
 'recall': 0.9386004321502684,
 'support': 46743}
### report_full
macro
{'f1-score': 0.7703889595978485,
 'precision': 0.8299850421272025,
 'recall': 0.7289496099583855,
 'support': 47168}
micro
{'f1-score': 0.934352738230878,
 'precision': 0.9386004321502684,
 'recall': 0.9301433175033921,
 'support': 47168}
weighted
{'f1-score': 0.9307512210509064,
 'precision': 0.9358913492006132,
 'recall': 0.9301433175033921,
 'support': 47168}
## test
PPCR: 0.990718
### report
macro
{'f1-score': 0.739545860037644,
 'precision': 0.801822364233868,
 'recall': 0.7036477331192513,
 'support': 11207}
micro
{'f1-score': 0.9039885785669671,
 'precision': 0.9039885785669671,
 'recall': 0.9039885785669671,
 'support': 11207}
weighted
{'f1-score': 0.8995084391941052,
 'precision': 0.9027765963295546,
 'recall': 0.9039885785669671,
 'support': 11207}
### report_full
macro
{'f1-score': 0.7317382703430187,
 'precision': 0.801822364233868,
 'recall': 0.6921402583362778,
 'support': 11312}
micro
{'f1-score': 0.8997735245792443,
 'precision': 0.9039885785669671,
 'recall': 0.8955975954738331,
 'support': 11312}
weighted
{'f1-score': 0.8945285320338074,
 'precision': 0.9024617452553121,
 'recall': 0.8955975954738331,
 'support': 11312}
```

## javascript
### Summary
138 rules, avg.len. 7.1

| | |
|-|-|
|Min support|136|
|Max support|19185|
|Min confidence|0.9215246438980103|
|Max confidence|0.9994054436683655|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {EXPRESSION}<br>	∧ -2.length ≤ 3<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 206.` |
| 2 | `  +1.internal_type = StringLiteral<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.980. Support: 470.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 276.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 277.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 200.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +5.length ≥ 3<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.942. Support: 197.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +5.length ≤ 2<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 211.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 4<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 180.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ -4.label not in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 292.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.label in {<-space>}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label not in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 274.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.label not in {<-space>}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label not in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 13673.` |
| 12 | `  ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2391.` |
| 13 | `  -1.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 807.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.944. Support: 189.` |
| 15 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 370.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 2650.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.987. Support: 837.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 744.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 201.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = export<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎⏎<br>Confidence: 0.942. Support: 181.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 218.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 8<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 446.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 538.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 4297.` |
| 25 | `  -1.diff_offset ≥ 2<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 213.` |
| 26 | `  -1.diff_offset ≥ 2<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +2.reserved = =<br>	∧ +5.roles in {INCOMPLETE}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.990. Support: 144.` |
| 27 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 265.` |
| 28 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 277.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 196.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≥ 6<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.991. Support: 165.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 283.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 4<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 160.` |
| 33 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.999. Support: 722.` |
| 34 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 187.` |
| 35 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎⏎<br>Confidence: 0.925. Support: 208.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 148.` |
| 37 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.962. Support: 171.` |
| 38 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 382.` |
| 39 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 370.` |
| 40 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.988. Support: 784.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 776.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.950. Support: 568.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2850.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CALL, CONDITION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 137.` |
| 45 | `  -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 822.` |
| 46 | `  -1.label not in {<space>}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.999. Support: 455.` |
| 47 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.999. Support: 512.` |
| 48 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.995. Support: 1051.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 330.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 245.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, >}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1343.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 285.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = export<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.978. Support: 204.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 152.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 679.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_offset ≤ 11<br>	∧ -3.reserved not in {.}<br>	∧ -5.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 714.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {, }}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {., ;}<br>	∧ -5.diff_offset ≥ 7<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 18377.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {., ;}<br>	∧ -5.diff_offset ≤ 6<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 478.` |
| 59 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = StringLiteral<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.997. Support: 190.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 197.` |
| 61 | `  •••start_col ≤ 44<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.967. Support: 169.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {.}<br>	∧ -4.diff_offset ≥ 21<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label not in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 14248.` |
| 64 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2444.` |
| 65 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.999. Support: 758.` |
| 66 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 199.` |
| 67 | `  •••start_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎⏎<br>Confidence: 0.947. Support: 238.` |
| 68 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.957. Support: 173.` |
| 69 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 396.` |
| 70 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 378.` |
| 71 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.981. Support: 796.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 745.` |
| 73 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.989. Support: 494.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 1<br>	∧ +3.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {CONDITION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2761.` |
| 75 | `  -1.diff_col ≥ 2<br>	∧ -1.roles in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 209.` |
| 76 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 253.` |
| 77 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 258.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 209.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 155.` |
| 80 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.949. Support: 205.` |
| 81 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 399.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.980. Support: 323.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 571.` |
| 84 | `  -1.roles in {STRING}<br>	∧ -2.length ≤ 3<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 204.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>	∧ ^2.internal_type = JSXElement<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.962. Support: 173.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 281.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 149.` |
| 88 | `  ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2477.` |
| 89 | `  -1.label in {<space>}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 829.` |
| 90 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.982. Support: 785.` |
| 91 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 184.` |
| 92 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.922. Support: 223.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 403.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.945. Support: 174.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 380.` |
| 96 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles in {EXPRESSION} and not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 2328.` |
| 97 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles not in {EXPRESSION, INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 669.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 826.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.988. Support: 561.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CONDITION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 2839.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 170.` |
| 102 | `  -1.length ≥ 2<br>	∧ -2.reserved = =<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 211.` |
| 103 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.987. Support: 197.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 136.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 390.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.954. Support: 164.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 415.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CONDITION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2776.` |
| 109 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.946. Support: 175.` |
| 110 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.999. Support: 496.` |
| 111 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.993. Support: 1007.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1184.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 8<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 683.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {., ;}<br>	∧ -5.diff_offset ≥ 7<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 19185.` |
| 115 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type = StringLiteral<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 204.` |
| 116 | `  -1.label in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.997. Support: 823.` |
| 117 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.987. Support: 825.` |
| 118 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 180.` |
| 119 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎⏎<br>Confidence: 0.968. Support: 203.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 429.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 393.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 788.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 227.` |
| 124 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.932. Support: 578.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 142.` |
| 126 | `  -1.diff_col ≥ 2<br>	∧ -2.reserved = =<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 212.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 177.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ -4.label not in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 245.` |
| 129 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 841.` |
| 130 | `  •••start_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.940. Support: 208.` |
| 131 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 385.` |
| 132 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 364.` |
| 133 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.979. Support: 794.` |
| 134 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 777.` |
| 135 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 204.` |
| 136 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.933. Support: 559.` |
| 137 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {CONDITION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2870.` |
| 138 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {CONDITION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 159.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.130434782608695, "max_conf": 0.9994054436683655, "max_support": 19185, "min_conf": 0.9215246438980103, "min_support": 136, "num_rules": 138}}
```
</details>
