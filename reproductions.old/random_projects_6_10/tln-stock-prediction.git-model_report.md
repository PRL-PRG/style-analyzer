# Model report for file:///tmp/top-repos-quality-repos-wlippkoz/tln-stock-prediction.git HEAD 3f694d35b68aa8fd8e8a6fb9b6d0025c16fdb6d1

### Dump

```json
{'created_at': '2021-08-21 11:30:02',
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
 'size': '16.0 kB',
 'tags': [],
 'uuid': '6dfe7f04-a3d7-4cd9-86e6-dd8e517f25a1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-wlippkoz/tln-stock-prediction.git 3f694d35b68aa8fd8e8a6fb9b6d0025c16fdb6d1

# javascript
13 rules, avg.len. 6.5
## train
PPCR: 0.761482
### report
macro
{'f1-score': 0.804211093011582,
 'precision': 0.8123401158944527,
 'recall': 0.7979324015553891,
 'support': 3863}
micro
{'f1-score': 0.9546984209163862,
 'precision': 0.9546984209163862,
 'recall': 0.9546984209163862,
 'support': 3863}
weighted
{'f1-score': 0.9517346168781081,
 'precision': 0.9502345750778686,
 'recall': 0.9546984209163862,
 'support': 3863}
### report_full
macro
{'f1-score': 0.6644315751401085,
 'precision': 0.8123401158944527,
 'recall': 0.578755516772275,
 'support': 5073}
micro
{'f1-score': 0.8254252461951656,
 'precision': 0.9546984209163862,
 'recall': 0.7269860043366844,
 'support': 5073}
weighted
{'f1-score': 0.8057857354835344,
 'precision': 0.9377872253436204,
 'recall': 0.7269860043366844,
 'support': 5073}
## test
PPCR: 0.721585
### report
macro
{'f1-score': 0.8007677389606982,
 'precision': 0.824647874909655,
 'recall': 0.7827452614223498,
 'support': 692}
micro
{'f1-score': 0.9364161849710982,
 'precision': 0.9364161849710982,
 'recall': 0.9364161849710982,
 'support': 692}
weighted
{'f1-score': 0.9317917905075939,
 'precision': 0.9315917805703842,
 'recall': 0.9364161849710982,
 'support': 692}
### report_full
macro
{'f1-score': 0.6521383041905712,
 'precision': 0.824647874909655,
 'recall': 0.5623153122655228,
 'support': 959}
micro
{'f1-score': 0.7849788007268322,
 'precision': 0.9364161849710982,
 'recall': 0.67570385818561,
 'support': 959}
weighted
{'f1-score': 0.7616262092523691,
 'precision': 0.9185379831651854,
 'recall': 0.67570385818561,
 'support': 959}
```

## javascript
### Summary
10 rules, avg.len. 6.0

| | |
|-|-|
|Min support|104|
|Max support|1015|
|Min confidence|0.9246305227279663|
|Max confidence|0.9983659982681274|

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
| 1 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.995. Support: 108.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1015.` |
| 3 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.992. Support: 323.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>⇒ y = '<br>Confidence: 0.998. Support: 239.` |
| 5 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 306.` |
| 6 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.roles in {MAP} and not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 193.` |
| 7 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ -2.label in {<newline>}<br>	∧ +1.roles in {IDENTIFIER} and not in {LITERAL, MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 149.` |
| 8 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT} and not in {IDENTIFIER, LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 117.` |
| 9 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {IDENTIFIER, LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {:, {}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {IDENTIFIER, LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 104.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9983659982681274, "max_support": 1015, "min_conf": 0.9246305227279663, "min_support": 104, "num_rules": 10}}
```
</details>
