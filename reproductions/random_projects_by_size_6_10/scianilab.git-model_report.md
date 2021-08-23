# Model report for file:///tmp/top-repos-quality-repos-x0qwt7e6/scianilab.git HEAD 71ccc5cf057a1233f1b93df203435437abaeb14c

### Dump

```json
{'created_at': '2021-08-21 11:04:15',
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
 'size': '23.8 kB',
 'tags': [],
 'uuid': '5d29d305-4365-4a10-a60d-97d13e5ebe24',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-x0qwt7e6/scianilab.git 71ccc5cf057a1233f1b93df203435437abaeb14c

# javascript
75 rules, avg.len. 8.3
## train
PPCR: 0.843611
### report
macro
{'f1-score': 0.2416083152857369,
 'precision': 0.25632790615256995,
 'recall': 0.23125936360987082,
 'support': 31972}
micro
{'f1-score': 0.9089203052671088,
 'precision': 0.9089203052671088,
 'recall': 0.9089203052671088,
 'support': 31972}
weighted
{'f1-score': 0.8996916274346031,
 'precision': 0.8927245943667426,
 'recall': 0.9089203052671088,
 'support': 31972}
### report_full
macro
{'f1-score': 0.19433012924637214,
 'precision': 0.25632790615256995,
 'recall': 0.1769432739562724,
 'support': 37899}
micro
{'f1-score': 0.8318186372028453,
 'precision': 0.9089203052671088,
 'recall': 0.7667748489406053,
 'support': 37899}
weighted
{'f1-score': 0.7738755421326023,
 'precision': 0.7966501306213752,
 'recall': 0.7667748489406053,
 'support': 37899}
## test
PPCR: 0.842991
### report
macro
{'f1-score': 0.20468663928196595,
 'precision': 0.23847282841887438,
 'recall': 0.19307718648124458,
 'support': 11522}
micro
{'f1-score': 0.8810970317653185,
 'precision': 0.8810970317653185,
 'recall': 0.8810970317653185,
 'support': 11522}
weighted
{'f1-score': 0.8677187132519547,
 'precision': 0.86830377804843,
 'recall': 0.8810970317653185,
 'support': 11522}
### report_full
macro
{'f1-score': 0.1740970921279073,
 'precision': 0.23847282841887438,
 'recall': 0.16371512177948805,
 'support': 13668}
micro
{'f1-score': 0.8060341405319571,
 'precision': 0.8810970317653185,
 'recall': 0.742756804214223,
 'support': 13668}
weighted
{'f1-score': 0.7486809008721342,
 'precision': 0.7814561240210947,
 'recall': 0.742756804214223,
 'support': 13668}
```

## javascript
### Summary
36 rules, avg.len. 6.2

| | |
|-|-|
|Min support|148|
|Max support|5324|
|Min confidence|0.9204014539718628|
|Max confidence|0.9993122220039368|

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
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.979. Support: 5324.` |
| 2 | `  +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 2340.` |
| 3 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.971. Support: 227.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 243.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 217.` |
| 6 | `  -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 917.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 2541.` |
| 8 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 613.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 913.` |
| 10 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 727.` |
| 11 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 177.` |
| 12 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 5312.` |
| 13 | `  +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 2408.` |
| 14 | `  -1.reserved = ;<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.length ≤ 32<br>	∧ ^1.roles in {FOR} and not in {BLOCK, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 152.` |
| 15 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.976. Support: 234.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.reserved = [<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 208.` |
| 17 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.internal_type = Identifier<br>	∧ -2.reserved not in {[}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 196.` |
| 18 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.internal_type = AssignmentExpression<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 2099.` |
| 19 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 584.` |
| 20 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles in {BLOCK} and not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 154.` |
| 21 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 201.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 2935.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {;, =}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 3637.` |
| 24 | `  -1.reserved = ;<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.length ≤ 36<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 148.` |
| 25 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 3082.` |
| 26 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 5306.` |
| 27 | `  +1.reserved = ;<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 2432.` |
| 28 | `  •••start_col ≥ 14<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles in {FOR} and not in {BLOCK, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 177.` |
| 29 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.984. Support: 223.` |
| 30 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 3056.` |
| 31 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 575.` |
| 32 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {]}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 190.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.reserved not in {(, ;, ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 2881.` |
| 34 | `  -1.reserved = ;<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.length ≤ 33<br>	∧ ^1.roles in {FOR} and not in {BLOCK, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 158.` |
| 35 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 3014.` |
| 36 | `  •••start_col ≥ 14<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles in {FOR} and not in {QUALIFIED, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 164.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.222222222222222, "max_conf": 0.9993122220039368, "max_support": 5324, "min_conf": 0.9204014539718628, "min_support": 148, "num_rules": 36}}
```
</details>
