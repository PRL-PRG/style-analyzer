# Model report for file:///tmp/top-repos-quality-repos-h1ss_7yc/rov-gui.git HEAD a725c430eed689736c456696e2a69f1be8b8121c

### Dump

```json
{'created_at': '2021-08-25 03:42:11',
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
 'size': '16.8 kB',
 'tags': [],
 'uuid': '76e29ed2-f3d0-4e1f-8873-580ecabf5766',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-h1ss_7yc/rov-gui.git a725c430eed689736c456696e2a69f1be8b8121c

# javascript
29 rules, avg.len. 5.8
## train
PPCR: 0.901613
### report
macro
{'f1-score': 0.3099159853371679,
 'precision': 0.31209909254029755,
 'recall': 0.32009170444080326,
 'support': 6653}
micro
{'f1-score': 0.8122651435442657,
 'precision': 0.8122651435442657,
 'recall': 0.8122651435442657,
 'support': 6653}
weighted
{'f1-score': 0.7676031334903802,
 'precision': 0.7496735567775029,
 'recall': 0.8122651435442657,
 'support': 6653}
### report_full
macro
{'f1-score': 0.3004191905676852,
 'precision': 0.31209909254029755,
 'recall': 0.30366725935801564,
 'support': 7379}
micro
{'f1-score': 0.7702394526795895,
 'precision': 0.8122651435442657,
 'recall': 0.732348556715002,
 'support': 7379}
weighted
{'f1-score': 0.7033937009921685,
 'precision': 0.7053561222219351,
 'recall': 0.732348556715002,
 'support': 7379}
## test
PPCR: 0.912379
### report
macro
{'f1-score': 0.29111094813909943,
 'precision': 0.2755203404905699,
 'recall': 0.31076417314776916,
 'support': 1135}
micro
{'f1-score': 0.7286343612334801,
 'precision': 0.7286343612334801,
 'recall': 0.7286343612334801,
 'support': 1135}
weighted
{'f1-score': 0.6842934816047822,
 'precision': 0.6481164252000214,
 'recall': 0.7286343612334801,
 'support': 1135}
### report_full
macro
{'f1-score': 0.2787579291397936,
 'precision': 0.2755203404905699,
 'recall': 0.2838968539044373,
 'support': 1244}
micro
{'f1-score': 0.6952501050861707,
 'precision': 0.7286343612334801,
 'recall': 0.6647909967845659,
 'support': 1244}
weighted
{'f1-score': 0.6377210534395631,
 'precision': 0.6169753182330561,
 'recall': 0.6647909967845659,
 'support': 1244}
```

## javascript
### Summary
3 rules, avg.len. 4.3

| | |
|-|-|
|Min support|147|
|Max support|2465|
|Min confidence|0.9231237173080444|
|Max confidence|0.9353741407394409|

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
| 1 | `  -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.934. Support: 158.` |
| 2 | `  -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.935. Support: 147.` |
| 3 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 2465.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.333333333333333, "max_conf": 0.9353741407394409, "max_support": 2465, "min_conf": 0.9231237173080444, "min_support": 147, "num_rules": 3}}
```
</details>
