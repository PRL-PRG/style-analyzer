# Model report for file:///tmp/top-repos-quality-repos-396j0jvo/pose-animator.git HEAD cb2be70a3501e57e66fe185daca44cc2afe18ce8

### Dump

```json
{'created_at': '2021-08-30 03:20:55',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': 'b949f760-3b61-4e12-bff4-f8e4b4391c6c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-396j0jvo/pose-animator.git cb2be70a3501e57e66fe185daca44cc2afe18ce8

# javascript
8 rules, avg.len. 4.4
## train
PPCR: 0.822536
### report
macro
{'f1-score': 0.3793169267370463,
 'precision': 0.40762110004132146,
 'recall': 0.3696625639112531,
 'support': 7059}
micro
{'f1-score': 0.9344099730840062,
 'precision': 0.9344099730840062,
 'recall': 0.9344099730840062,
 'support': 7059}
weighted
{'f1-score': 0.923213544210606,
 'precision': 0.918955988556671,
 'recall': 0.9344099730840062,
 'support': 7059}
### report_full
macro
{'f1-score': 0.34475407846146555,
 'precision': 0.40762110004132146,
 'recall': 0.30723536719486433,
 'support': 8582}
micro
{'f1-score': 0.8434243334825139,
 'precision': 0.9344099730840062,
 'recall': 0.7685854113260312,
 'support': 8582}
weighted
{'f1-score': 0.8124039703383035,
 'precision': 0.8678041078521646,
 'recall': 0.7685854113260312,
 'support': 8582}
## test
PPCR: 0.894568
### report
macro
{'f1-score': 0.37935830978759505,
 'precision': 0.4140473342362932,
 'recall': 0.3634693417599891,
 'support': 5846}
micro
{'f1-score': 0.9197742045843311,
 'precision': 0.9197742045843311,
 'recall': 0.9197742045843311,
 'support': 5846}
weighted
{'f1-score': 0.9124524818085719,
 'precision': 0.9247103373321026,
 'recall': 0.9197742045843311,
 'support': 5846}
### report_full
macro
{'f1-score': 0.349719262331406,
 'precision': 0.4140473342362932,
 'recall': 0.3177988929726728,
 'support': 6535}
micro
{'f1-score': 0.8685889669655117,
 'precision': 0.9197742045843311,
 'recall': 0.8228003060443765,
 'support': 6535}
weighted
{'f1-score': 0.8476456394198003,
 'precision': 0.9009400472703784,
 'recall': 0.8228003060443765,
 'support': 6535}
```

## javascript
### Summary
4 rules, avg.len. 4.8

| | |
|-|-|
|Min support|104|
|Max support|1729|
|Min confidence|0.9673221707344055|
|Max confidence|0.9962173104286194|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1454.` |
| 2 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 279.` |
| 3 | `  -1.roles in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 104.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 1729.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.75, "max_conf": 0.9962173104286194, "max_support": 1729, "min_conf": 0.9673221707344055, "min_support": 104, "num_rules": 4}}
```
</details>
