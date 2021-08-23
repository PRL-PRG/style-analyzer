# Model report for file:///tmp/top-repos-quality-repos-y77l8pv8/ce5508-wally.git HEAD 4754ebe118e0967bde3475738afd51977940470a

### Dump

```json
{'created_at': '2021-08-20 22:12:46',
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
 'size': '15.0 kB',
 'tags': [],
 'uuid': '32d0c622-8a42-4efb-8f0c-8772dfa0ac9e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-y77l8pv8/ce5508-wally.git 4754ebe118e0967bde3475738afd51977940470a

# javascript
8 rules, avg.len. 4.9
## train
PPCR: 0.663109
### report
macro
{'f1-score': 0.5907848025661805,
 'precision': 0.5930403354017414,
 'recall': 0.590906096368424,
 'support': 2547}
micro
{'f1-score': 0.9140164899882215,
 'precision': 0.9140164899882215,
 'recall': 0.9140164899882215,
 'support': 2547}
weighted
{'f1-score': 0.90216161697033,
 'precision': 0.8932343733899638,
 'recall': 0.9140164899882215,
 'support': 2547}
### report_full
macro
{'f1-score': 0.48391994072817474,
 'precision': 0.5930403354017414,
 'recall': 0.42562932072639015,
 'support': 3841}
micro
{'f1-score': 0.7288666249217283,
 'precision': 0.9140164899882215,
 'recall': 0.6060921634990888,
 'support': 3841}
weighted
{'f1-score': 0.6830622492978591,
 'precision': 0.823797175561659,
 'recall': 0.6060921634990888,
 'support': 3841}
## test
PPCR: 0.586478
### report
macro
{'f1-score': 0.5509269230193984,
 'precision': 0.5282776164722865,
 'recall': 0.5813947366441782,
 'support': 373}
micro
{'f1-score': 0.8364611260053619,
 'precision': 0.8364611260053619,
 'recall': 0.8364611260053619,
 'support': 373}
weighted
{'f1-score': 0.8233263816136821,
 'precision': 0.8144472948880989,
 'recall': 0.8364611260053619,
 'support': 373}
### report_full
macro
{'f1-score': 0.4382321628237777,
 'precision': 0.5282776164722865,
 'recall': 0.38356098890665297,
 'support': 636}
micro
{'f1-score': 0.6184340931615461,
 'precision': 0.8364611260053619,
 'recall': 0.49056603773584906,
 'support': 636}
weighted
{'f1-score': 0.5860271288824951,
 'precision': 0.7443566082156721,
 'recall': 0.49056603773584906,
 'support': 636}
```

## javascript
### Summary
4 rules, avg.len. 5.2

| | |
|-|-|
|Min support|92|
|Max support|604|
|Min confidence|0.929347813129425|
|Max confidence|0.9968553185462952|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 159.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 92.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 604.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 372.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.25, "max_conf": 0.9968553185462952, "max_support": 604, "min_conf": 0.929347813129425, "min_support": 92, "num_rules": 4}}
```
</details>
