# Model report for file:///tmp/top-repos-quality-repos-l_vk29hn/sidoine.org.git HEAD ed247b93ac65ff3f878ddf81ccacff09282ed1b1

### Dump

```json
{'created_at': '2021-08-22 18:17:34',
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
 'size': '14.5 kB',
 'tags': [],
 'uuid': '0a16e911-1e4a-4aca-80be-0cda2669fe65',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-l_vk29hn/sidoine.org.git ed247b93ac65ff3f878ddf81ccacff09282ed1b1

# javascript
4 rules, avg.len. 5.0
## train
PPCR: 0.676182
### report
macro
{'f1-score': 0.44687403315190616,
 'precision': 0.4562148709997705,
 'recall': 0.44258373205741625,
 'support': 2203}
micro
{'f1-score': 0.8942351339083069,
 'precision': 0.8942351339083069,
 'recall': 0.8942351339083069,
 'support': 2203}
weighted
{'f1-score': 0.857610087970216,
 'precision': 0.8280944117750497,
 'recall': 0.8942351339083069,
 'support': 2203}
### report_full
macro
{'f1-score': 0.3109095113817044,
 'precision': 0.4562148709997705,
 'recall': 0.26450189882416975,
 'support': 3258}
micro
{'f1-score': 0.7214795824940488,
 'precision': 0.8942351339083069,
 'recall': 0.6046654389195826,
 'support': 3258}
weighted
{'f1-score': 0.6406441544450043,
 'precision': 0.7871043694535271,
 'recall': 0.6046654389195826,
 'support': 3258}
## test
PPCR: 0.625402
### report
macro
{'f1-score': 0.4214483139856274,
 'precision': 0.4635012919896641,
 'recall': 0.40277777777777785,
 'support': 389}
micro
{'f1-score': 0.9511568123393316,
 'precision': 0.9511568123393316,
 'recall': 0.9511568123393316,
 'support': 389}
weighted
{'f1-score': 0.9329266265075652,
 'precision': 0.9201590243319185,
 'recall': 0.9511568123393316,
 'support': 389}
### report_full
macro
{'f1-score': 0.27663252647453196,
 'precision': 0.4635012919896641,
 'recall': 0.23918477706356492,
 'support': 622}
micro
{'f1-score': 0.7319485657764591,
 'precision': 0.9511568123393316,
 'recall': 0.594855305466238,
 'support': 622}
weighted
{'f1-score': 0.637195224912497,
 'precision': 0.8593341060345471,
 'recall': 0.594855305466238,
 'support': 622}
```

## javascript
### Summary
2 rules, avg.len. 4.5

| | |
|-|-|
|Min support|108|
|Max support|849|
|Min confidence|0.952296793460846|
|Max confidence|0.9953703880310059|

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
                     'min_samples_leaf': 95,
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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.995. Support: 108.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, MODULE}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 849.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.9953703880310059, "max_support": 849, "min_conf": 0.952296793460846, "min_support": 108, "num_rules": 2}}
```
</details>
