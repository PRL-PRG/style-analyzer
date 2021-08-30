# Model report for file:///tmp/top-repos-quality-repos-xhjr11z0/house.git HEAD c3641e300d9c40568cd57dce1884f1a96c4c7849

### Dump

```json
{'created_at': '2021-08-30 04:40:56',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': '472f5e3f-20ac-41de-9b45-aa563a35d0db',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xhjr11z0/house.git c3641e300d9c40568cd57dce1884f1a96c4c7849

# javascript
12 rules, avg.len. 4.8
## train
PPCR: 0.629705
### report
macro
{'f1-score': 0.5867172839354569,
 'precision': 0.5850294995318507,
 'recall': 0.5895397041777798,
 'support': 5588}
micro
{'f1-score': 0.9443450250536864,
 'precision': 0.9443450250536864,
 'recall': 0.9443450250536864,
 'support': 5588}
weighted
{'f1-score': 0.9378898886568486,
 'precision': 0.9327176141118539,
 'recall': 0.9443450250536864,
 'support': 5588}
### report_full
macro
{'f1-score': 0.48073272320671373,
 'precision': 0.5850294995318507,
 'recall': 0.4175954598518551,
 'support': 8874}
micro
{'f1-score': 0.7297745816622874,
 'precision': 0.9443450250536864,
 'recall': 0.594658553076403,
 'support': 8874}
weighted
{'f1-score': 0.6997238226740146,
 'precision': 0.8579103551698916,
 'recall': 0.594658553076403,
 'support': 8874}
## test
PPCR: 0.741379
### report
macro
{'f1-score': 0.5468332907210438,
 'precision': 0.5232822149828982,
 'recall': 0.5769902712781907,
 'support': 1978}
micro
{'f1-score': 0.9661274014155713,
 'precision': 0.9661274014155713,
 'recall': 0.9661274014155713,
 'support': 1978}
weighted
{'f1-score': 0.9646257057892927,
 'precision': 0.9643949262985989,
 'recall': 0.9661274014155713,
 'support': 1978}
### report_full
macro
{'f1-score': 0.4623481128024575,
 'precision': 0.5232822149828982,
 'recall': 0.4279090428357359,
 'support': 2668}
micro
{'f1-score': 0.8226431338786052,
 'precision': 0.9661274014155713,
 'recall': 0.7162668665667167,
 'support': 2668}
weighted
{'f1-score': 0.8133937558179091,
 'precision': 0.9458643067806579,
 'recall': 0.7162668665667167,
 'support': 2668}
```

## javascript
### Summary
7 rules, avg.len. 4.7

| | |
|-|-|
|Min support|121|
|Max support|2294|
|Min confidence|0.9413793087005615|
|Max confidence|0.9964285492897034|

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
| 1 | `  -2.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 166.` |
| 2 | `  •••start_line ≤ 1<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.992. Support: 318.` |
| 3 | `  •••start_line ≤ 1<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.986. Support: 253.` |
| 4 | `  •••start_line ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 140.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {'}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 2294.` |
| 6 | `  •••start_col ≥ 21<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 145.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 121.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.714285714285714, "max_conf": 0.9964285492897034, "max_support": 2294, "min_conf": 0.9413793087005615, "min_support": 121, "num_rules": 7}}
```
</details>
