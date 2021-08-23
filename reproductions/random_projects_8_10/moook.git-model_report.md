# Model report for file:///tmp/top-repos-quality-repos-bra8avhe/moook.git HEAD 8f36afb2074ada471327aef79b119ded832e9a38

### Dump

```json
{'created_at': '2021-08-21 00:08:16',
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
 'size': '20.4 kB',
 'tags': [],
 'uuid': 'e115eb3d-b436-468b-b795-36652f1a95d2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-bra8avhe/moook.git 8f36afb2074ada471327aef79b119ded832e9a38

# javascript
13 rules, avg.len. 6.2
## train
PPCR: 0.871182
### report
macro
{'f1-score': 0.2917518638281544,
 'precision': 0.30042549957882164,
 'recall': 0.2867980998455633,
 'support': 11436}
micro
{'f1-score': 0.9367785939139559,
 'precision': 0.9367785939139559,
 'recall': 0.9367785939139559,
 'support': 11436}
weighted
{'f1-score': 0.9280964110444715,
 'precision': 0.9222283039428063,
 'recall': 0.9367785939139559,
 'support': 11436}
### report_full
macro
{'f1-score': 0.24039231472727032,
 'precision': 0.30042549957882164,
 'recall': 0.21638270310310537,
 'support': 13127}
micro
{'f1-score': 0.8722875870211293,
 'precision': 0.9367785939139559,
 'recall': 0.8161042126913994,
 'support': 13127}
weighted
{'f1-score': 0.827863681829714,
 'precision': 0.8585087555692958,
 'recall': 0.8161042126913994,
 'support': 13127}
## test
PPCR: 0.870840
### report
macro
{'f1-score': 0.2905924612599608,
 'precision': 0.27905397210898597,
 'recall': 0.30854400354686584,
 'support': 1544}
micro
{'f1-score': 0.9203367875647669,
 'precision': 0.9203367875647669,
 'recall': 0.9203367875647669,
 'support': 1544}
weighted
{'f1-score': 0.9113288503906525,
 'precision': 0.9048548137050946,
 'recall': 0.9203367875647669,
 'support': 1544}
### report_full
macro
{'f1-score': 0.24105428536657905,
 'precision': 0.27905397210898597,
 'recall': 0.22286553218539798,
 'support': 1773}
micro
{'f1-score': 0.8567983117274647,
 'precision': 0.9203367875647669,
 'recall': 0.8014664410603497,
 'support': 1773}
weighted
{'f1-score': 0.8134062548856945,
 'precision': 0.8343625362792438,
 'recall': 0.8014664410603497,
 'support': 1773}
```

## javascript
### Summary
5 rules, avg.len. 4.0

| | |
|-|-|
|Min support|163|
|Max support|2308|
|Min confidence|0.9662576913833618|
|Max confidence|0.9987593293190002|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 2308.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 403.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BODY}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 163.` |
| 4 | `  +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 514.` |
| 5 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1564.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9987593293190002, "max_support": 2308, "min_conf": 0.9662576913833618, "min_support": 163, "num_rules": 5}}
```
</details>
