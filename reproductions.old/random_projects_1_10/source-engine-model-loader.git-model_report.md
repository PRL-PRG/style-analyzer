# Model report for file:///tmp/top-repos-quality-repos-v4hlockb/source-engine-model-loader.git HEAD e90f2a62fcae645742df40dae29fd65fe7529811

### Dump

```json
{'created_at': '2021-08-22 11:56:36',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.8 kB',
 'tags': [],
 'uuid': '06202800-9040-4b4d-b1df-8b6c243f5446',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v4hlockb/source-engine-model-loader.git e90f2a62fcae645742df40dae29fd65fe7529811

# javascript
10 rules, avg.len. 4.2
## train
PPCR: 0.876804
### report
macro
{'f1-score': 0.6775308345048876,
 'precision': 0.670066890323335,
 'recall': 0.6870123769505992,
 'support': 11601}
micro
{'f1-score': 0.9497457115765883,
 'precision': 0.9497457115765883,
 'recall': 0.9497457115765883,
 'support': 11601}
weighted
{'f1-score': 0.9436166961729969,
 'precision': 0.9379319441586945,
 'recall': 0.9497457115765883,
 'support': 11601}
### report_full
macro
{'f1-score': 0.5940569009850128,
 'precision': 0.670066890323335,
 'recall': 0.566505332181477,
 'support': 13231}
micro
{'f1-score': 0.8874033505154639,
 'precision': 0.9497457115765883,
 'recall': 0.8327412893961151,
 'support': 13231}
weighted
{'f1-score': 0.8443344964576833,
 'precision': 0.8658413830837862,
 'recall': 0.8327412893961151,
 'support': 13231}
## test
PPCR: 0.862773
### report
macro
{'f1-score': 0.6618653532732272,
 'precision': 0.6733345077239087,
 'recall': 0.6546599372577802,
 'support': 1817}
micro
{'f1-score': 0.9411117226197028,
 'precision': 0.9411117226197028,
 'recall': 0.9411117226197028,
 'support': 1817}
weighted
{'f1-score': 0.9320518513680884,
 'precision': 0.9243298542846273,
 'recall': 0.9411117226197028,
 'support': 1817}
### report_full
macro
{'f1-score': 0.5755657759797815,
 'precision': 0.6733345077239087,
 'recall': 0.5457561925075637,
 'support': 2106}
micro
{'f1-score': 0.8717817996431303,
 'precision': 0.9411117226197028,
 'recall': 0.811965811965812,
 'support': 2106}
weighted
{'f1-score': 0.827513222968716,
 'precision': 0.8561153005195372,
 'recall': 0.811965811965812,
 'support': 2106}
```

## javascript
### Summary
8 rules, avg.len. 4.1

| | |
|-|-|
|Min support|102|
|Max support|4585|
|Min confidence|0.9404761791229248|
|Max confidence|0.9971910119056702|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 1995.` |
| 2 | `  +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 869.` |
| 3 | `  +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 615.` |
| 4 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁻<br>Confidence: 0.995. Support: 102.` |
| 5 | `  •••start_col ≤ 12<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.940. Support: 126.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 525.` |
| 7 | `  -1.reserved not in {;}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type = CommentLine<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 8 | `  -1.reserved not in {;, {, }}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 4585.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.125, "max_conf": 0.9971910119056702, "max_support": 4585, "min_conf": 0.9404761791229248, "min_support": 102, "num_rules": 8}}
```
</details>
