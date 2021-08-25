# Model report for file:///tmp/top-repos-quality-repos-h854ovbm/lime.git HEAD 152bd3df2cf661bd6251e2a3533dbc410ee7d2fd

### Dump

```json
{'created_at': '2021-08-25 05:44:21',
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
 'size': '15.3 kB',
 'tags': [],
 'uuid': 'b6834e2a-5f15-4338-9fba-9d0177b95391',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-h854ovbm/lime.git 152bd3df2cf661bd6251e2a3533dbc410ee7d2fd

# javascript
8 rules, avg.len. 5.5
## train
PPCR: 0.627083
### report
macro
{'f1-score': 0.2682935810087283,
 'precision': 0.25837114787805865,
 'recall': 0.2795250013266418,
 'support': 3274}
micro
{'f1-score': 0.9346365302382407,
 'precision': 0.9346365302382407,
 'recall': 0.9346365302382407,
 'support': 3274}
weighted
{'f1-score': 0.9129400883662904,
 'precision': 0.8932352637947331,
 'recall': 0.9346365302382407,
 'support': 3274}
### report_full
macro
{'f1-score': 0.21519047101525382,
 'precision': 0.25837114787805865,
 'recall': 0.19104581135666024,
 'support': 5221}
micro
{'f1-score': 0.7204237786933491,
 'precision': 0.9346365302382407,
 'recall': 0.5860946178892933,
 'support': 5221}
weighted
{'f1-score': 0.6411489504774474,
 'precision': 0.7348320983928125,
 'recall': 0.5860946178892933,
 'support': 5221}
## test
PPCR: 0.656885
### report
macro
{'f1-score': 0.25455099199443093,
 'precision': 0.2427365264736053,
 'recall': 0.2677165354330709,
 'support': 582}
micro
{'f1-score': 0.8917525773195877,
 'precision': 0.8917525773195877,
 'recall': 0.8917525773195877,
 'support': 582}
weighted
{'f1-score': 0.8550722420975955,
 'precision': 0.8215552831289183,
 'recall': 0.8917525773195877,
 'support': 582}
### report_full
macro
{'f1-score': 0.2137694110171174,
 'precision': 0.2427365264736053,
 'recall': 0.19716601573999568,
 'support': 886}
micro
{'f1-score': 0.7070844686648501,
 'precision': 0.8917525773195877,
 'recall': 0.5857787810383747,
 'support': 886}
weighted
{'f1-score': 0.6182577216654712,
 'precision': 0.6752568292306265,
 'recall': 0.5857787810383747,
 'support': 886}
```

## javascript
### Summary
6 rules, avg.len. 4.2

| | |
|-|-|
|Min support|91|
|Max support|865|
|Min confidence|0.9462427496910095|
|Max confidence|0.9948275685310364|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 865.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 91.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 561.` |
| 4 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 290.` |
| 5 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 184.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 107.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.166666666666667, "max_conf": 0.9948275685310364, "max_support": 865, "min_conf": 0.9462427496910095, "min_support": 91, "num_rules": 6}}
```
</details>
