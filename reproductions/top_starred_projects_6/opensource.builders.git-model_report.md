# Model report for file:///tmp/top-repos-quality-repos-thr8jp55/opensource.builders.git HEAD 28b6a2c6d42127b16e790913df0140981ecf3c64

### Dump

```json
{'created_at': '2021-08-29 20:54:38',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '14.7 kB',
 'tags': [],
 'uuid': '4284b367-3166-4bb0-a287-69652ec576c4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-thr8jp55/opensource.builders.git 28b6a2c6d42127b16e790913df0140981ecf3c64

# javascript
6 rules, avg.len. 5.5
## train
PPCR: 0.675681
### report
macro
{'f1-score': 0.45438772410446887,
 'precision': 0.47027564474528233,
 'recall': 0.4464219815558878,
 'support': 3748}
micro
{'f1-score': 0.9178228388473852,
 'precision': 0.9178228388473852,
 'recall': 0.9178228388473852,
 'support': 3748}
weighted
{'f1-score': 0.8944001884873051,
 'precision': 0.8788408813286817,
 'recall': 0.9178228388473852,
 'support': 3748}
### report_full
macro
{'f1-score': 0.3676795345710166,
 'precision': 0.47027564474528233,
 'recall': 0.3348710194785341,
 'support': 5547}
micro
{'f1-score': 0.7401828940290478,
 'precision': 0.9178228388473852,
 'recall': 0.6201550387596899,
 'support': 5547}
weighted
{'f1-score': 0.6565404363591463,
 'precision': 0.7790236884719616,
 'recall': 0.6201550387596899,
 'support': 5547}
## test
PPCR: 0.671084
### report
macro
{'f1-score': 0.41357818973518173,
 'precision': 0.47977899167717153,
 'recall': 0.3958333333333333,
 'support': 557}
micro
{'f1-score': 0.9102333931777378,
 'precision': 0.9102333931777379,
 'recall': 0.9102333931777379,
 'support': 557}
weighted
{'f1-score': 0.8801952443806369,
 'precision': 0.8751866373957555,
 'recall': 0.9102333931777379,
 'support': 557}
### report_full
macro
{'f1-score': 0.32622990297785665,
 'precision': 0.47977899167717153,
 'recall': 0.3053571428571429,
 'support': 830}
micro
{'f1-score': 0.7310742609949531,
 'precision': 0.9102333931777379,
 'recall': 0.6108433734939759,
 'support': 830}
weighted
{'f1-score': 0.6318500892074858,
 'precision': 0.8332726176395433,
 'recall': 0.6108433734939759,
 'support': 830}
```

## javascript
### Summary
4 rules, avg.len. 4.2

| | |
|-|-|
|Min support|112|
|Max support|290|
|Min confidence|0.9422110319137573|
|Max confidence|0.998275876045227|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.998. Support: 290.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.942. Support: 199.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 112.` |
| 4 | `  -1.reserved not in {,, :}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.internal_type = Identifier<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 128.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.25, "max_conf": 0.998275876045227, "max_support": 290, "min_conf": 0.9422110319137573, "min_support": 112, "num_rules": 4}}
```
</details>
