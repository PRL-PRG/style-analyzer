# Model report for file:///tmp/top-repos-quality-repos-2a9r2gzr/travelroute.git HEAD 63478bb0511bccf4a75f93834978f093d5b54f47

### Dump

```json
{'created_at': '2021-08-20 00:58:50',
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
 'size': '14.7 kB',
 'tags': [],
 'uuid': '0e4ffcbb-4f5f-423b-8983-b3c936f33982',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2a9r2gzr/travelroute.git 63478bb0511bccf4a75f93834978f093d5b54f47

# javascript
7 rules, avg.len. 4.0
## train
PPCR: 0.680035
### report
macro
{'f1-score': 0.3198812802262836,
 'precision': 0.3132781847535151,
 'recall': 0.32700846198403305,
 'support': 3103}
micro
{'f1-score': 0.955849178214631,
 'precision': 0.955849178214631,
 'recall': 0.955849178214631,
 'support': 3103}
weighted
{'f1-score': 0.945117847692484,
 'precision': 0.9350765013796348,
 'recall': 0.955849178214631,
 'support': 3103}
### report_full
macro
{'f1-score': 0.26934297887429043,
 'precision': 0.3132781847535151,
 'recall': 0.23874211881955212,
 'support': 4563}
micro
{'f1-score': 0.773806417949387,
 'precision': 0.955849178214631,
 'recall': 0.6500109577032654,
 'support': 4563}
weighted
{'f1-score': 0.7150211867926498,
 'precision': 0.802721294612464,
 'recall': 0.6500109577032654,
 'support': 4563}
## test
PPCR: 0.629338
### report
macro
{'f1-score': 0.31028274867024136,
 'precision': 0.3029252704031465,
 'recall': 0.3182181092360733,
 'support': 399}
micro
{'f1-score': 0.949874686716792,
 'precision': 0.949874686716792,
 'recall': 0.949874686716792,
 'support': 399}
weighted
{'f1-score': 0.9382099217291889,
 'precision': 0.9270983505962547,
 'recall': 0.949874686716792,
 'support': 399}
### report_full
macro
{'f1-score': 0.23013491330665845,
 'precision': 0.3029252704031465,
 'recall': 0.19721999062730133,
 'support': 634}
micro
{'f1-score': 0.73378509196515,
 'precision': 0.949874686716792,
 'recall': 0.5977917981072555,
 'support': 634}
weighted
{'f1-score': 0.6650719027354207,
 'precision': 0.793647115751468,
 'recall': 0.5977917981072555,
 'support': 634}
```

## javascript
### Summary
4 rules, avg.len. 4.5

| | |
|-|-|
|Min support|129|
|Max support|1022|
|Min confidence|0.9263566136360168|
|Max confidence|0.9907044768333435|

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
                     'min_samples_split': 185,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved not in {;}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1022.` |
| 2 | `  -1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.926. Support: 129.` |
| 3 | `  -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 456.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 199.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.9907044768333435, "max_support": 1022, "min_conf": 0.9263566136360168, "min_support": 129, "num_rules": 4}}
```
</details>
