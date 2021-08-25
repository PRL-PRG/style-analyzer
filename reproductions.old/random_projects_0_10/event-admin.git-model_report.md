# Model report for file:///tmp/top-repos-quality-repos-t42palmv/event-admin.git HEAD 774b6aa49cd816b004202f065c8e14ecaf707898

### Dump

```json
{'created_at': '2021-08-22 21:23:12',
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
 'size': '14.6 kB',
 'tags': [],
 'uuid': '3c824fae-e9ec-4977-8f11-1e5874bf40f7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t42palmv/event-admin.git 774b6aa49cd816b004202f065c8e14ecaf707898

# javascript
6 rules, avg.len. 4.3
## train
PPCR: 0.614908
### report
macro
{'f1-score': 0.4211678426964733,
 'precision': 0.4678771520876784,
 'recall': 0.40577342047930287,
 'support': 2681}
micro
{'f1-score': 0.897053338306602,
 'precision': 0.897053338306602,
 'recall': 0.897053338306602,
 'support': 2681}
weighted
{'f1-score': 0.8686996588433229,
 'precision': 0.8665677229645897,
 'recall': 0.897053338306602,
 'support': 2681}
### report_full
macro
{'f1-score': 0.3330143112322221,
 'precision': 0.4678771520876784,
 'recall': 0.29623935473758345,
 'support': 4360}
micro
{'f1-score': 0.6831415992046584,
 'precision': 0.897053338306602,
 'recall': 0.551605504587156,
 'support': 4360}
weighted
{'f1-score': 0.5987555124446494,
 'precision': 0.7661418534788889,
 'recall': 0.551605504587156,
 'support': 4360}
## test
PPCR: 0.636531
### report
macro
{'f1-score': 0.45125623321825853,
 'precision': 0.4760900140646976,
 'recall': 0.44097222222222227,
 'support': 345}
micro
{'f1-score': 0.9014492753623189,
 'precision': 0.9014492753623189,
 'recall': 0.9014492753623189,
 'support': 345}
weighted
{'f1-score': 0.8753174563466253,
 'precision': 0.866311991683483,
 'recall': 0.9014492753623189,
 'support': 345}
### report_full
macro
{'f1-score': 0.3564670787662174,
 'precision': 0.4760900140646976,
 'recall': 0.32011265050415677,
 'support': 542}
micro
{'f1-score': 0.7012401352874859,
 'precision': 0.9014492753623189,
 'recall': 0.5738007380073801,
 'support': 542}
weighted
{'f1-score': 0.6248570732433377,
 'precision': 0.8012595948744298,
 'recall': 0.5738007380073801,
 'support': 542}
```

## javascript
### Summary
5 rules, avg.len. 4.2

| | |
|-|-|
|Min support|117|
|Max support|201|
|Min confidence|0.9273504018783569|
|Max confidence|0.9975124597549438|

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
                     'min_samples_leaf': 106,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.997. Support: 188.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.996. Support: 118.` |
| 3 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.927. Support: 117.` |
| 4 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.998. Support: 201.` |
| 5 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 177.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.2, "max_conf": 0.9975124597549438, "max_support": 201, "min_conf": 0.9273504018783569, "min_support": 117, "num_rules": 5}}
```
</details>
