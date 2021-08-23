# Model report for file:///tmp/top-repos-quality-repos-1n4rdwaq/django-webpage.git HEAD bfeebd3080481690b8736fbd5fe5e8003912a5b2

### Dump

```json
{'created_at': '2021-08-21 11:24:32',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': 'e3558d01-7977-44c3-ad5f-894890869850',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1n4rdwaq/django-webpage.git bfeebd3080481690b8736fbd5fe5e8003912a5b2

# javascript
11 rules, avg.len. 6.4
## train
PPCR: 0.888947
### report
macro
{'f1-score': 0.892820955237903,
 'precision': 0.9358210474809753,
 'recall': 0.8581245782835079,
 'support': 5083}
micro
{'f1-score': 0.9073381861105646,
 'precision': 0.9073381861105646,
 'recall': 0.9073381861105646,
 'support': 5083}
weighted
{'f1-score': 0.9053218151370908,
 'precision': 0.9077326934106218,
 'recall': 0.9073381861105646,
 'support': 5083}
### report_full
macro
{'f1-score': 0.8162734994709605,
 'precision': 0.9358210474809753,
 'recall': 0.7439523791876556,
 'support': 5718}
micro
{'f1-score': 0.85399500046292,
 'precision': 0.9073381861105646,
 'recall': 0.8065757257782441,
 'support': 5718}
weighted
{'f1-score': 0.8425260805398458,
 'precision': 0.9053428844980523,
 'recall': 0.8065757257782441,
 'support': 5718}
## test
PPCR: 0.892308
### report
macro
{'f1-score': 0.8819310617697713,
 'precision': 0.9609647539997593,
 'recall': 0.8372366835821986,
 'support': 232}
micro
{'f1-score': 0.9008620689655172,
 'precision': 0.9008620689655172,
 'recall': 0.9008620689655172,
 'support': 232}
weighted
{'f1-score': 0.8950757274797875,
 'precision': 0.9043370997648054,
 'recall': 0.9008620689655172,
 'support': 232}
### report_full
macro
{'f1-score': 0.8072241630734691,
 'precision': 0.9609647539997593,
 'recall': 0.7349149388623072,
 'support': 260}
micro
{'f1-score': 0.8495934959349594,
 'precision': 0.9008620689655172,
 'recall': 0.8038461538461539,
 'support': 260}
weighted
{'f1-score': 0.8337521940496163,
 'precision': 0.9069677705910113,
 'recall': 0.8038461538461539,
 'support': 260}
```

## javascript
### Summary
6 rules, avg.len. 4.3

| | |
|-|-|
|Min support|135|
|Max support|1643|
|Min confidence|0.9294871687889099|
|Max confidence|0.9981549978256226|

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
                     'min_samples_leaf': 98,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.998. Support: 271.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 2<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.929. Support: 234.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 150.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 135.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 158.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 1643.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.333333333333333, "max_conf": 0.9981549978256226, "max_support": 1643, "min_conf": 0.9294871687889099, "min_support": 135, "num_rules": 6}}
```
</details>
